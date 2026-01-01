#!/usr/bin/env python3
"""
Run_Oracles.py
Compiles and runs test oracles for generated A3 Rust pairs.

For each directory containing:
  - A3_fixed.rs
  - A3_vulnerable.rs
  - demo_test.rs  (or oracle.rs/orcale.rs)

We verify the contract per file:
  * A3_fixed.rs       SHOULD PASS the tests  → if not, delete ONLY A3_fixed.rs
  * A3_vulnerable.rs  SHOULD FAIL the tests  → if not, delete ONLY A3_vulnerable.rs

CLI:
  --delete-bad   : delete the specific file(s) that violate the contract (default: True)
  --dry-run      : print what would be deleted but do not delete

Notes:
- We NEVER delete directories or demo_test.rs; only offending A3_*.rs files.
- A compile timeout/failure counts as a violation for that file (and will be deleted).
"""

import os
import sys
import shutil
import subprocess
import tempfile
from pathlib import Path
from typing import Dict, Any, List, Tuple

DEFAULT_ROOT = "/home/ikqnm/PycharmProjects/rudra_sandbox/rudra_big/rudra_big/src/dataset/o3_mini/data_generation/CWE-190"
RUSTC_EDITION = "2021"
TIMEOUT_SECS = 600  # seconds per compile/run

# Ensure rust tools are on PATH
os.environ["PATH"] = os.path.expanduser("~/.cargo/bin") + os.pathsep + os.environ.get("PATH", "")

def check_cmd_exists(cmd: str) -> None:
    if shutil.which(cmd) is None:
        print(f"ERROR: '{cmd}' not found in PATH.", file=sys.stderr)
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
    with tempfile.TemporaryDirectory(prefix="a3_oracle_") as td:
        td_path = Path(td)
        combined = td_path / f"{rs_file.stem}_with_oracle.rs"
        try:
            src_code = _read_text_with_fallback(rs_file)
            oracle_code = _read_text_with_fallback(oracle_file)
        except Exception as e:
            return {'compiled': False, 'ran': False, 'ok': False, 'phase': 'read', 'output': str(e), 'bin': ''}

        combined.write_text(src_code + "\n\n" + oracle_code + "\n", encoding="utf-8")

        out_bin = td_path / (rs_file.stem + ("_tests.exe" if os.name == "nt" else "_tests"))
        cmd = [
            "rustc", "--edition", RUSTC_EDITION, "--test", str(combined),
            "-C", "debuginfo=0", "-O", "-o", str(out_bin)
        ]
        try:
            comp = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=TIMEOUT_SECS)
        except subprocess.TimeoutExpired:
            return {'compiled': False, 'ran': False, 'ok': False, 'phase': 'compile', 'output': 'compile timeout', 'bin': str(out_bin)}
        if comp.returncode != 0:
            return {'compiled': False, 'ran': False, 'ok': False, 'phase': 'compile', 'output': comp.stdout or '', 'bin': str(out_bin)}

        try:
            run = subprocess.run([str(out_bin), "--color", "never"], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=TIMEOUT_SECS)
        except subprocess.TimeoutExpired:
            return {'compiled': True, 'ran': False, 'ok': False, 'phase': 'run', 'output': 'test timeout'}

        return {'compiled': True, 'ran': True, 'ok': (run.returncode == 0), 'phase': 'run', 'output': run.stdout or ''}

def contract_decisions(fixed_res: Dict[str, Any], vuln_res: Dict[str, Any]) -> Tuple[bool, bool, str, str]:
    """
    Decide per-file contract:
      - fixed_ok   = compiled & ran & ok (PASS)
      - vuln_fail  = compiled & ran & NOT ok (FAIL)

    Returns:
      (fixed_ok, vuln_fail, fixed_reason_if_bad, vuln_reason_if_bad)
    """
    # Fixed expectations
    fixed_ok = fixed_res['compiled'] and fixed_res['ran'] and fixed_res['ok']
    if not fixed_res['compiled']:
        fixed_reason = f"compile failed ({fixed_res.get('phase')})"
    elif not fixed_res['ran']:
        fixed_reason = "did not run (timeout/error)"
    elif not fixed_res['ok']:
        fixed_reason = "tests failed (expected PASS)"
    else:
        fixed_reason = ""

    # Vulnerable expectations
    vuln_fail = vuln_res['compiled'] and vuln_res['ran'] and (not vuln_res['ok'])
    if not vuln_res['compiled']:
        vuln_reason = f"compile failed ({vuln_res.get('phase')})"
    elif not vuln_res['ran']:
        vuln_reason = "did not run (timeout/error)"
    elif vuln_res['ok']:
        vuln_reason = "tests passed (expected FAIL)"
    else:
        vuln_reason = ""

    return fixed_ok, vuln_fail, fixed_reason, vuln_reason

def pick_test_file(filenames: set, dirpath: Path) -> Path | None:
    # preference order
    for name in ("demo_test.rs", "oracle.rs", "orcale.rs"):
        if name in filenames:
            return dirpath / name
    return None

def process_root(root: Path, delete_bad: bool = True, dry_run: bool = False):
    scanned = 0
    fixed_passed = 0
    vuln_failed_as_expected = 0

    will_delete_files: List[Tuple[Path, str]] = []
    deleted_files: List[Path] = []

    for dirpath, _dirnames, filenames in os.walk(root):
        files = set(filenames)
        if not {"A3_fixed.rs", "A3_vulnerable.rs"}.issubset(files):
            continue

        test_path = pick_test_file(files, Path(dirpath))
        if test_path is None:
            continue

        p = Path(dirpath)
        scanned += 1
        print(f"\n=== {p.relative_to(root)} ===")

        fixed_path = p / "A3_fixed.rs"
        vuln_path  = p / "A3_vulnerable.rs"

        f_res = compile_and_run_single(fixed_path, test_path)
        v_res = compile_and_run_single(vuln_path,  test_path)

        print(f"[fixed] compiled={f_res['compiled']} ran={f_res['ran']} ok={f_res['ok']} phase={f_res['phase']}")
        if f_res.get('output'):
            print("---- fixed logs ----")
            print(f_res['output'])
        print(f"[vuln ] compiled={v_res['compiled']} ran={v_res['ran']} ok={v_res['ok']} phase={v_res['phase']}")
        if v_res.get('output'):
            print("---- vuln logs ----")
            print(v_res['output'])

        fixed_ok, vuln_fail, fixed_reason, vuln_reason = contract_decisions(f_res, v_res)

        if fixed_ok:
            fixed_passed += 1
        if vuln_fail:
            vuln_failed_as_expected += 1

        # Per-file decisions
        del_fixed = not fixed_ok
        del_vuln  = not vuln_fail

        if not del_fixed and not del_vuln:
            print("[decision] KEEP (A3_fixed PASSED, A3_vulnerable FAILED as expected)")
        else:
            print("[decision] BAD (contract not met)")
            if del_fixed:
                print(f"  -> {'would delete' if dry_run or not delete_bad else 'delete'}: {fixed_path.name}  (reason: {fixed_reason})")
                will_delete_files.append((fixed_path, fixed_reason))
            if del_vuln:
                print(f"  -> {'would delete' if dry_run or not delete_bad else 'delete'}: {vuln_path.name}   (reason: {vuln_reason})")
                will_delete_files.append((vuln_path, vuln_reason))

    # Perform deletions if requested (and not dry-run)
    if delete_bad and not dry_run and will_delete_files:
        print("\n[cleanup] deleting files that violate the contract:")
        for f, reason in will_delete_files:
            try:
                if f.exists():
                    f.unlink()
                    deleted_files.append(f)
                    print(f"  removed: {f.relative_to(root)}  ({reason})")
            except Exception as e:
                print(f"  failed to remove {f}: {e}", file=sys.stderr)

    # Summary
    print("\n[summary]")
    print(f"  directories scanned            : {scanned}")
    print(f"  fixed PASSED (as expected)     : {fixed_passed}")
    print(f"  vulnerable FAILED (as expected): {vuln_failed_as_expected}")
    print(f"  files flagged for deletion     : {len(will_delete_files)}")
    print(f"  files actually deleted         : {len(deleted_files)}")
    if deleted_files:
        print("  deleted list:")
        for i, f in enumerate(deleted_files, 1):
            print(f"    {i:>2}. {f.relative_to(root)}")

def main():
    import argparse
    ap = argparse.ArgumentParser(description="Compile and run tests for generated A3 pairs (delete only offending files).")
    ap.add_argument("--root", default=DEFAULT_ROOT, help="Root folder containing generated outputs")
    ap.add_argument("--delete-bad", action="store_true", default=True,
                    help="Delete files that fail the per-file contract (default: True)")
    ap.add_argument("--dry-run", action="store_true", help="Print what would be deleted, but do not delete")
    args = ap.parse_args()

    root = Path(args.root).expanduser().resolve()
    if not root.exists():
        print(f"ERROR: root not found: {root}", file=sys.stderr)
        sys.exit(1)

    check_cmd_exists("rustc")
    print(f"[info] Using: {subprocess.check_output(['rustc', '--version'], text=True).strip()}")
    process_root(root, delete_bad=args.delete_bad, dry_run=args.dry_run)

if __name__ == "__main__":
    main()
