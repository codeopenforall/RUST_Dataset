#!/usr/bin/env python3
import os
import sys
import shutil
import subprocess
import tempfile
import re
import time
from pathlib import Path
from typing import Tuple, Dict, Any

# Base directory to scan
ROOT = Path("/home/ikqnm/PycharmProjects/PythonProject/O3_mini/DataGenerated_pairs_new/CWE-020")

# rustc settings
RUSTC_EDITION = "2021"
TIMEOUT_SECS = 600  # per compile

# Ensure ~/.cargo/bin is on PATH (useful when running from IDE/venv)
os.environ["PATH"] = os.path.expanduser("~/.cargo/bin") + os.pathsep + os.environ.get("PATH", "")

MAIN_RE = re.compile(r'^\s*fn\s+main\s*\(', re.MULTILINE)

def check_cmd_exists(cmd: str) -> None:
    if shutil.which(cmd) is None:
        print(f"ERROR: '{cmd}' not found in PATH. Please install it and retry.", file=sys.stderr)
        sys.exit(1)

def detect_crate_type(rs_file: Path) -> str:
    """
    Decide whether to compile as a binary ('bin') or library ('lib').
    If the file contains a top-level `fn main(...)`, treat as bin; otherwise lib.
    """
    try:
        text = rs_file.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        text = rs_file.read_text(encoding="latin-1")
    return "bin" if MAIN_RE.search(text) else "lib"

def compile_single(rs_file: Path) -> Dict[str, Any]:
    """
    Compile a single Rust source file (no test running).
    Returns a dict with:
      ok: bool
      output: str (stdout+stderr)
      crate_type: 'bin' | 'lib'
      duration_secs: float
      timeout: bool
      cmd: list[str]
      artifact: str (temp path used)
    """
    crate_type = detect_crate_type(rs_file)

    with tempfile.TemporaryDirectory(prefix="rust_compile_") as td:
        td_path = Path(td)
        if crate_type == "bin":
            out_artifact = td_path / (rs_file.stem + (".exe" if os.name == "nt" else ""))
            compile_cmd = [
                "rustc",
                "--edition", RUSTC_EDITION,
                str(rs_file),
                "-C", "debuginfo=0",
                "-O",
                "-o", str(out_artifact),
            ]
        else:
            out_artifact = td_path / (rs_file.stem + ".rlib")
            compile_cmd = [
                "rustc",
                "--edition", RUSTC_EDITION,
                "--crate-type", "lib",
                str(rs_file),
                "-C", "debuginfo=0",
                "-O",
                "-o", str(out_artifact),
            ]

        t0 = time.time()
        try:
            comp = subprocess.run(
                compile_cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                timeout=TIMEOUT_SECS,
            )
            timeout = False
        except subprocess.TimeoutExpired:
            comp = subprocess.CompletedProcess(compile_cmd, returncode=124, stdout="compile timeout")
            timeout = True
        dt = time.time() - t0

        ok = (comp.returncode == 0)
        return {
            "ok": ok,
            "output": (comp.stdout or "").strip(),
            "crate_type": crate_type,
            "duration_secs": dt,
            "timeout": timeout,
            "cmd": compile_cmd,
            "artifact": str(out_artifact),
        }

def main():
    check_cmd_exists("rustc")

    if not ROOT.exists():
        print(f"ERROR: root directory not found: {ROOT}", file=sys.stderr)
        sys.exit(1)

    rustc_ver = subprocess.check_output(['rustc', '--version'], text=True).strip()
    print(f"[info] Using rustc: {rustc_ver}")
    print(f"[info] PATH head: {os.environ.get('PATH','').split(os.pathsep)[0]}")
    print(f"[info] Scanning root: {ROOT}\n")

    total_files = 0
    compiled_ok = 0
    dirs_seen = 0
    dirs_marked_for_deletion = []

    for dirpath, _dirnames, filenames in os.walk(ROOT):
        p = Path(dirpath)
        rs_targets = []
        has_fixed = "fixed.rs" in filenames
        has_vuln  = "vulnerable.rs" in filenames
        if has_fixed:
            rs_targets.append(p / "fixed.rs")
        if has_vuln:
            rs_targets.append(p / "vulnerable.rs")
        if not rs_targets:
            continue

        dirs_seen += 1
        print(f"=== Directory: {p.relative_to(ROOT)} ===")
        print(f"  contains: fixed.rs={'yes' if has_fixed else 'no'}, vulnerable.rs={'yes' if has_vuln else 'no'}")

        dir_failed = False

        for rs_file in rs_targets:
            total_files += 1
            res = compile_single(rs_file)
            status = "OK" if res["ok"] else "FAIL"
            print(f"  -> {rs_file.name} [{res['crate_type']}], {status}, {res['duration_secs']:.2f}s")
            print(f"     cmd: {' '.join(res['cmd'])}")
            print(f"     artifact: {res['artifact']}")
            if res["output"]:
                print("     --- compiler output begin ---")
                for line in res["output"].splitlines():
                    print(f"     {line}")
                print("     --- compiler output end ---")
            else:
                print("     (no compiler output)")

            if res["ok"]:
                compiled_ok += 1
            else:
                dir_failed = True

        if dir_failed:
            dirs_marked_for_deletion.append(p)
            print("  decision: DELETE (one or more files failed to compile)")
        else:
            print("  decision: KEEP (all files compiled)")

        print()

    if dirs_seen == 0:
        print("No 'fixed.rs' or 'vulnerable.rs' files were found under the root.\n")

    print(f"[summary] directories scanned (with targets): {dirs_seen}")
    print(f"[summary] files compiled OK: {compiled_ok}/{total_files}")

    # Perform deletions (delete dir and ALL contents)
    deleted_ok = 0
    delete_errors = 0
    unique_to_delete = sorted(set(dirs_marked_for_deletion), key=lambda x: len(x.parts), reverse=True)

    if unique_to_delete:
        print("\n[cleanup] Deleting directories with non-compilable files:")
        for d in unique_to_delete:
            try:
                shutil.rmtree(d)
                deleted_ok += 1
                print(f"  removed: {d}")
            except Exception as e:
                delete_errors += 1
                print(f"  failed to remove {d}: {e}", file=sys.stderr)
    else:
        print("\n[cleanup] No directories to delete.")

    # Final tallies
    marked = len(unique_to_delete)
    kept = dirs_seen - deleted_ok

    print("\n[final]")
    print(f"  directories marked for deletion: {marked}/{dirs_seen}")
    print(f"  directories successfully deleted: {deleted_ok}/{dirs_seen}")
    if delete_errors:
        print(f"  directories failed to delete (errors): {delete_errors}/{marked}")
    print(f"  directories kept: {kept}/{dirs_seen}")

    # Exit non-zero if any compilation failures (useful for CI)
    exit_code = 0 if compiled_ok == total_files and total_files > 0 else 2
    print(f"\n[exit] code: {exit_code}")
    sys.exit(exit_code)

if __name__ == "__main__":
    main()
