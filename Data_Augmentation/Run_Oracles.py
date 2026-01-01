#!/usr/bin/env python3
import os
import sys
import shutil
import subprocess
import tempfile
from pathlib import Path
from typing import Dict, Any, List

# Base directory to scan
ROOT = Path("/home/ikqnm/PycharmProjects/PythonProject/O3_mini/DataGenerated_pairs_new/CWE-190")

# rustc settings
RUSTC_EDITION = "2021"
TIMEOUT_SECS = 600  # per compile/run

# Ensure ~/.cargo/bin is on PATH (useful when running from IDE/venv)
os.environ["PATH"] = os.path.expanduser("~/.cargo/bin") + os.pathsep + os.environ.get("PATH", "")

def check_cmd_exists(cmd: str) -> None:
    if shutil.which(cmd) is None:
        print(f"ERROR: '{cmd}' not found in PATH. Please install it and retry.", file=sys.stderr)
        sys.exit(1)

def _read_text_with_fallback(p: Path) -> str:
    try:
        return p.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return p.read_text(encoding="latin-1")

def compile_and_run_single(rs_file: Path, oracle_file: Path) -> Dict[str, Any]:
    """
    Concatenate rs_file + oracle_file into a temporary source, then compile with `rustc --test`
    and run the produced test binary.

    Returns:
      {
        'compiled': bool,
        'ran': bool,
        'ok': bool,         # test returncode == 0
        'phase': str,       # 'compile' or 'run' (where the output came from)
        'output': str,      # stdout/stderr text
        'bin': str          # temp binary path
      }
    """
    with tempfile.TemporaryDirectory(prefix="rust_inline_oracle_") as td:
        td_path = Path(td)

        # Build a combined source file: <target>.rs + oracle.rs
        combined_src = td_path / f"{rs_file.stem}_with_oracle.rs"
        try:
            src_code = _read_text_with_fallback(rs_file)
            oracle_code = _read_text_with_fallback(oracle_file)
        except Exception as e:
            return {
                'compiled': False,
                'ran': False,
                'ok': False,
                'phase': 'read',
                'output': f'failed to read sources: {e}',
                'bin': ''
            }

        combined_src.write_text(src_code + "\n\n" + oracle_code + "\n", encoding="utf-8")

        out_bin = td_path / (rs_file.stem + ("_tests.exe" if os.name == "nt" else "_tests"))

        compile_cmd = [
            "rustc",
            "--edition", RUSTC_EDITION,
            "--test",
            str(combined_src),
            "-C", "debuginfo=0",
            "-O",
            "-o", str(out_bin),
        ]

        try:
            comp = subprocess.run(
                compile_cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                timeout=TIMEOUT_SECS,
            )
        except subprocess.TimeoutExpired:
            return {'compiled': False, 'ran': False, 'ok': False, 'phase': 'compile', 'output': 'compile timeout', 'bin': str(out_bin)}

        if comp.returncode != 0:
            return {'compiled': False, 'ran': False, 'ok': False, 'phase': 'compile', 'output': (comp.stdout or '').strip(), 'bin': str(out_bin)}

        # Run tests
        try:
            run = subprocess.run(
                [str(out_bin), "--color", "never"],
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                timeout=TIMEOUT_SECS,
            )
        except subprocess.TimeoutExpired:
            return {'compiled': True, 'ran': False, 'ok': False, 'phase': 'run', 'output': 'test timeout', 'bin': str(out_bin)}

        return {
            'compiled': True,
            'ran': True,
            'ok': (run.returncode == 0),
            'phase': 'run',
            'output': (run.stdout or '').strip(),
            'bin': str(out_bin),
        }

def main():
    check_cmd_exists("rustc")

    if not ROOT.exists():
        print(f"ERROR: root directory not found: {ROOT}", file=sys.stderr)
        sys.exit(1)

    print(f"[info] Using rustc: {subprocess.check_output(['rustc', '--version'], text=True).strip()}")
    print(f"[info] Scanning: {ROOT}\n")

    # Directory-level stats
    dirs_scanned = 0
    dirs_keep: List[Path] = []
    dirs_delete: List[Path] = []

    # File-level stats
    files_seen = 0
    files_compiled_ok = 0
    files_ran_ok = 0
    fixed_pass_count = 0
    vuln_fail_count = 0

    for dirpath, _dirnames, filenames in os.walk(ROOT):
        p = Path(dirpath)
        has_fixed = "fixed.rs" in filenames
        has_vuln  = "vulnerable.rs" in filenames

        # accept both 'oracle.rs' and common typo 'orcale.rs'
        oracle_name = "oracle.rs" if "oracle.rs" in filenames else ("orcale.rs" if "orcale.rs" in filenames else None)
        has_oracle = oracle_name is not None

        if not (has_fixed or has_vuln or has_oracle):
            continue

        dirs_scanned += 1
        print(f"=== Directory: {p.relative_to(ROOT)} ===")
        print(f"  contains: fixed.rs={'yes' if has_fixed else 'no'}, vulnerable.rs={'yes' if has_vuln else 'no'}, oracle={'yes' if has_oracle else 'no'}")

        # Must have all three files
        if not (has_fixed and has_vuln and has_oracle):
            print("  decision: DELETE (missing fixed.rs or vulnerable.rs or oracle.rs)")
            dirs_delete.append(p)
            print()
            continue

        fixed_path = p / "fixed.rs"
        vuln_path  = p / "vulnerable.rs"
        oracle_path = p / oracle_name

        # ----- fixed.rs + oracle.rs -----
        files_seen += 1
        fixed_res = compile_and_run_single(fixed_path, oracle_path)
        if fixed_res['compiled']:
            files_compiled_ok += 1
        if fixed_res['ran'] and fixed_res['ok']:
            files_ran_ok += 1
            fixed_pass_count += 1

        print(f"  -> fixed.rs: compiled={fixed_res['compiled']} ran={fixed_res['ran']} ok={fixed_res['ok']}")
        print("     --- fixed logs begin ---")
        for line in (fixed_res['output'] or "").splitlines():
            print(f"     {line}")
        print("     --- fixed logs end ---")

        # ----- vulnerable.rs + oracle.rs -----
        files_seen += 1
        vuln_res = compile_and_run_single(vuln_path, oracle_path)
        if vuln_res['compiled']:
            files_compiled_ok += 1
        if vuln_res['ran'] and vuln_res['ok']:
            files_ran_ok += 1
        # Expect vulnerable to FAIL (ok == False)
        if vuln_res['compiled'] and vuln_res['ran'] and (not vuln_res['ok']):
            vuln_fail_count += 1

        print(f"  -> vulnerable.rs: compiled={vuln_res['compiled']} ran={vuln_res['ran']} ok={vuln_res['ok']}")
        print("     --- vulnerable logs begin ---")
        for line in (vuln_res['output'] or "").splitlines():
            print(f"     {line}")
        print("     --- vulnerable logs end ---")

        # Contract: fixed must PASS and vulnerable must FAIL
        fixed_ok = fixed_res['compiled'] and fixed_res['ran'] and fixed_res['ok']
        vuln_fail = vuln_res['compiled'] and vuln_res['ran'] and (not vuln_res['ok'])

        if fixed_ok and vuln_fail:
            print("  decision: KEEP (fixed PASSED, vulnerable FAILED as expected)")
            dirs_keep.append(p)
        else:
            reasons = []
            if not fixed_ok:
                reasons.append("fixed did not PASS")
            if not vuln_fail:
                reasons.append("vulnerable did not FAIL")
            print(f"  decision: DELETE ({' and '.join(reasons) if reasons else 'unexpected state'})")
            dirs_delete.append(p)

        print()

    if dirs_scanned == 0:
        print("No directories with fixed.rs or vulnerable.rs were found.")
        print("\n[summary] scanned: 0, kept: 0, marked for deletion: 0, deleted: 0")
        sys.exit(0)

    # Delete dirs that failed the contract
    unique_delete = sorted(set(dirs_delete), key=lambda x: len(x.parts), reverse=True)
    deleted_count = 0
    if unique_delete:
        print("[cleanup] Deleting directories that do not meet expectations:")
        for d in unique_delete:
            try:
                shutil.rmtree(d)
                deleted_count += 1
                print(f"  removed: {d}")
            except Exception as e:
                print(f"  failed to remove {d}: {e}", file=sys.stderr)
    else:
        print("[cleanup] No directories to delete.")

    # Final summary
    kept_count = len(set(dirs_keep) - set(unique_delete))
    print("\n[summary]")
    print(f"  directories scanned: {dirs_scanned}")
    print(f"  directories kept: {kept_count}")
    print(f"  directories marked for deletion: {len(unique_delete)}")
    print(f"  directories deleted: {deleted_count} / {dirs_scanned}")
    print(f"  files seen: {files_seen}")
    print(f"  files compiled OK: {files_compiled_ok}/{files_seen}")
    print(f"  files ran OK (tests passed): {files_ran_ok}/{files_seen}")
    print(f"  fixed PASSED (as expected): {fixed_pass_count}")
    print(f"  vulnerable FAILED (as expected): {vuln_fail_count}")

    # Exit non-zero if any deletions occurred (i.e., any directories failed expectations)
    exit_code = 0 if deleted_count == 0 else 2
    print(f"\n[exit] code: {exit_code}")
    sys.exit(exit_code)

if __name__ == "__main__":
    main()
