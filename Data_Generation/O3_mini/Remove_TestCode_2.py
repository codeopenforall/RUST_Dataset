#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Remove_TestCode_2.py — compile & run tests for a single A2 pair (env-driven)
Checks contract for the current subdir:
  - A2_fixed.rs       SHOULD PASS tests
  - A2_vulnerable.rs  SHOULD FAIL tests
If A2_DELETE_BAD=1, deletes only the offending file(s). Optionally appends to CSV if A2_SUMMARY_CSV is set.

Environment (set by orchestrator):
  A2_DST_ROOT        -> absolute/relative path to destination root
  A2_CURRENT_SUBDIR  -> subdirectory name to process
  A2_SUMMARY_CSV     -> optional path to CSV for logging
  A2_DELETE_BAD      -> "1" to delete offending files, else keep
"""

import os
import sys
import shutil
import subprocess
import tempfile
from pathlib import Path
from typing import Dict, Any, Tuple, Optional

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
    """
    with tempfile.TemporaryDirectory(prefix="a2_oracle_") as td:
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
            return {'compiled': True, 'ran': False, 'ok': False, 'phase': 'run', 'output': 'test timeout', 'bin': str(out_bin)}

        return {'compiled': True, 'ran': True, 'ok': (run.returncode == 0), 'phase': 'run', 'output': run.stdout or '', 'bin': str(out_bin)}

def contract_decisions(fixed_res: Dict[str, Any], vuln_res: Dict[str, Any]) -> Tuple[bool, bool, str, str]:
    """
    Decide per-file contract:
      - fixed_ok   = compiled & ran & ok (PASS)
      - vuln_fail  = compiled & ran & NOT ok (FAIL)
    Returns (fixed_ok, vuln_fail, fixed_reason_if_bad, vuln_reason_if_bad).
    """
    fixed_ok = fixed_res['compiled'] and fixed_res['ran'] and fixed_res['ok']
    if not fixed_res['compiled']:
        fixed_reason = f"compile failed ({fixed_res.get('phase')})"
    elif not fixed_res['ran']:
        fixed_reason = "did not run (timeout/error)"
    elif not fixed_res['ok']:
        fixed_reason = "tests failed (expected PASS)"
    else:
        fixed_reason = ""

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

def append_summary(csv_path: Optional[str], subdir: str, target: str, res: Dict[str, Any]) -> None:
    if not csv_path:
        return
    import csv, time
    row = {
        "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
        "subdir": subdir,
        "target": target,
        "compiled": str(res.get("compiled", False)),
        "ran": str(res.get("ran", False)),
        "ok": str(res.get("ok", False)),
        "phase": str(res.get("phase", "")),
        "output_len": str(len(res.get("output", "") or "")),
    }
    write_header = not Path(csv_path).exists()
    with open(csv_path, "a", newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=list(row.keys()))
        if write_header:
            w.writeheader()
        w.writerow(row)

def main():
    # Env-only
    DST_ROOT = os.environ.get("A2_DST_ROOT")
    SUB = os.environ.get("A2_CURRENT_SUBDIR")
    CSV = os.environ.get("A2_SUMMARY_CSV", None)
    DELETE_BAD = os.environ.get("A2_DELETE_BAD", "0") == "1"

    if not (DST_ROOT and SUB):
        print("Missing env vars: require A2_DST_ROOT and A2_CURRENT_SUBDIR", file=sys.stderr)
        sys.exit(2)

    check_cmd_exists("rustc")
    print(f"[info] Using rustc: {subprocess.check_output(['rustc', '--version'], text=True).strip()}"

    )

    d = Path(DST_ROOT) / SUB
    if not d.exists():
        print(f"[warn] directory does not exist: {d}")
        sys.exit(0)

    # choose oracle file
    oracle = None
    for name in ("demo_test.rs", "oracle.rs"):
        cand = d / name
        if cand.exists():
            oracle = cand
            break
    if oracle is None:
        print(f"[warn] No test oracle present in {d}")
        sys.exit(0)

    fixed_path = d / "A2_fixed.rs"
    vuln_path  = d / "A2_vulnerable.rs"

    if not fixed_path.exists() and not vuln_path.exists():
        print(f"[info] No A2 files to test in {d}")
        sys.exit(0)

    # run tests
    f_res = compile_and_run_single(fixed_path, oracle) if fixed_path.exists() else {'compiled': False, 'ran': False, 'ok': False, 'phase': 'none', 'output': ''}
    v_res = compile_and_run_single(vuln_path,  oracle) if vuln_path.exists()  else {'compiled': False, 'ran': False, 'ok': True,  'phase': 'none', 'output': ''}

    append_summary(CSV, SUB, "fixed", f_res)
    append_summary(CSV, SUB, "vulnerable", v_res)

    print(f"[fixed] compiled={f_res['compiled']} ran={f_res['ran']} ok={f_res['ok']} phase={f_res['phase']}")
    if f_res.get('output'):
        print("---- fixed logs ----")
        print(f_res['output'])
    print(f"[vuln ] compiled={v_res['compiled']} ran={v_res['ran']} ok={v_res['ok']} phase={v_res['phase']}")
    if v_res.get('output'):
        print("---- vuln logs ----")
        print(v_res['output'])

    fixed_ok, vuln_fail, fixed_reason, vuln_reason = contract_decisions(f_res, v_res)

    # Decisions
    del_fixed = fixed_path.exists() and not fixed_ok
    del_vuln  = vuln_path.exists()  and not vuln_fail

    if not del_fixed and not del_vuln:
        print("[decision] KEEP (A2_fixed PASSED, A2_vulnerable FAILED as expected)")
        sys.exit(0)
    else:
        print("[decision] BAD")
        if del_fixed:
            print(f"  -> BAD fixed: {fixed_reason}")
        if del_vuln:
            print(f"  -> BAD vuln : {vuln_reason}")

        if DELETE_BAD:
            for f in [p for p in (fixed_path, vuln_path) if p and p.exists() and ((p==fixed_path and del_fixed) or (p==vuln_path and del_vuln))]:
                try:
                    f.unlink()
                    print(f"  removed: {f}")
                except Exception as e:
                    print(f"  failed to remove {f}: {e}", file=sys.stderr)

        # Return non-zero so orchestrator notices issues but continues
        sys.exit(1)

if __name__ == "__main__":
    main()