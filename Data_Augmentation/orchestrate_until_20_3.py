#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
orchestrate_until_20_3.py — controlled orchestrator (phased generation & validation)

Behavior (per user request):
  1) First generate 20 A3_fixed.rs files (if not already present)
  2) Then generate 20 A3_vulnerable.rs files (if not already present)
  3) Remove test code from both fixed and vulnerable outputs
  4) Run tests via Run_Oracles_3.py and delete files that don't meet criteria
  5) Repeat until total files (fixed + vulnerable + demo_test) >= 60

This script calls these helper scripts (they must be next to this file or on PATH):
  - A3.MakeFixed.py
  - A3.MakeVulnerable.py
  - Remove_TestCode_3.py
  - Run_Oracles_3.py

Configuration:
  SOURCE (input) and DEST (output) paths are set below.
  TARGET_FIXED = 20, TARGET_VULN = 20, TARGET_TOTAL = 60 (fixed+vuln+demo_test)
  MAX_PASSES to avoid infinite loops.
"""

import os, sys, time, subprocess
from pathlib import Path
from datetime import datetime
from typing import List, Tuple

# ---------- Paths & targets (edit if needed) ----------
SOURCE = Path("/home/ikqnm/PycharmProjects/PythonProject/O3_mini/Cleaned_DataGenerated_pairs_CC/CWE-190").resolve()
DEST   = Path("/home/ikqnm/PycharmProjects/PythonProject/deepseek/DataAugmented_pairs_new_A3/CWE-190").resolve()

TARGET_FIXED = int(os.environ.get("TARGET_FIXED", "20"))
TARGET_VULN  = int(os.environ.get("TARGET_VULN",  "20"))
TARGET_TOTAL = int(os.environ.get("TARGET_TOTAL", "60"))
MAX_PASSES   = int(os.environ.get("MAX_PASSES", "2"))

ONLY_MISSING = True   # don't re-generate outputs that already exist in DEST unless deleted by oracles
OVERWRITE_TEST = True

# ---------- Script names (expected next to this file or on PATH) ----------
SCRIPT_FIXED      = Path("A3.MakeFixed.py")
SCRIPT_VULNERABLE = Path("A3.MakeVulnerable.py")
SCRIPT_REMOVE     = Path("Remove_TestCode_3.py")
SCRIPT_ORACLES    = Path("Run_Oracles_3.py")

# ---------- Logging ----------
def log(msg: str, level: str="INFO"):
    ts = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    print(f"{ts} [{level}] {msg}")

# ---------- Discovery ----------
def dir_has_required(d: Path) -> bool:
    names = {p.name for p in d.iterdir() if p.is_file()}
    return ("fixed_diff.rs" in names) and ("demo_test.rs" in names)

def find_candidates(root: Path) -> List[Path]:
    out = []
    if not root.exists():
        return out
    for d in sorted([p for p in root.rglob("*") if p.is_dir()]):
        try:
            if dir_has_required(d):
                out.append(d)
        except Exception:
            continue
    return out

# ---------- Counting outputs ----------
def count_outputs(dest_root: Path) -> Tuple[int,int,int,int]:
    fixed = vuln = tests = 0
    if not dest_root.exists():
        return 0,0,0,0
    for d in sorted([p for p in dest_root.rglob("*") if p.is_dir()]):
        names = {p.name for p in d.iterdir() if p.is_file()}
        if "A3_fixed.rs" in names: fixed += 1
        if "A3_vulnerable.rs" in names: vuln += 1
        if "demo_test.rs" in names: tests += 1
    total = fixed + vuln + tests
    return fixed, vuln, tests, total

# ---------- Helpers ----------
def rel_from_root(p: Path, root: Path) -> str:
    try: return str(p.relative_to(root))
    except Exception: return p.name

def run_cmd(cmd: List[str], env: dict, cwd: Path, timeout: int=3600) -> int:
    log(f"RUN: {' '.join(cmd)}  (cwd={cwd})")
    try:
        res = subprocess.run(cmd, cwd=str(cwd), env=env, capture_output=True, text=True, timeout=timeout)
    except Exception as e:
        log(f"Command execution failed: {e}", "ERROR")
        return 127
    if res.stdout:
        log("stdout:\n" + res.stdout.strip(), "INFO")
    if res.stderr:
        log("stderr:\n" + res.stderr.strip(), "ERROR")
    return res.returncode

def script_path_or_cwd(script: Path) -> Path:
    if script.is_file():
        return script
    local = Path.cwd()/script
    return local if local.is_file() else script  # may be on PATH

# ---------- Phases ----------
def generate_fixed(candidates: List[Path], src_root: Path, dest_root: Path) -> None:
    fixed_count, _, _, _ = count_outputs(dest_root)
    if fixed_count >= TARGET_FIXED:
        log("Already have target fixed files; skipping fixed generation.")
        return
    for src in candidates:
        fixed_count, _, _, _ = count_outputs(dest_root)
        if fixed_count >= TARGET_FIXED:
            break
        rel = rel_from_root(src, src_root)
        out_dir = dest_root / rel
        out_dir.mkdir(parents=True, exist_ok=True)
        if ONLY_MISSING and (out_dir/"A3_fixed.rs").exists():
            log(f"Skip fixed (exists): {rel}")
            continue
        env = os.environ.copy()
        env.update({
            "A3_SRC_ROOT": str(src_root),
            "A3_DST_ROOT": str(dest_root),
            "A3_CURRENT_SUBDIR": rel,
            "A3_ONLY_MISSING": "1" if ONLY_MISSING else "0",
            "A3_OVERWRITE_TEST": "1" if OVERWRITE_TEST else "0",
        })
        fixed_script = str(script_path_or_cwd(SCRIPT_FIXED))
        rc = run_cmd([sys.executable, fixed_script], env, cwd=Path.cwd())
        log(f"MakeFixed returned {rc} for {rel}")
        time.sleep(0.2)

def generate_vuln(candidates: List[Path], src_root: Path, dest_root: Path) -> None:
    _, vuln_count, _, _ = count_outputs(dest_root)
    if vuln_count >= TARGET_VULN:
        log("Already have target vulnerable files; skipping vulnerable generation.")
        return
    for src in candidates:
        _, vuln_count, _, _ = count_outputs(dest_root)
        if vuln_count >= TARGET_VULN:
            break
        rel = rel_from_root(src, src_root)
        out_dir = dest_root / rel
        out_dir.mkdir(parents=True, exist_ok=True)
        if ONLY_MISSING and (out_dir/"A3_vulnerable.rs").exists():
            log(f"Skip vuln (exists): {rel}")
            continue
        env = os.environ.copy()
        env.update({
            "A3V_SRC_ROOT": str(src_root),
            "A3V_DST_ROOT": str(dest_root),
            "A3V_CURRENT_SUBDIR": rel,
            "A3V_ONLY_MISSING": "1" if ONLY_MISSING else "0",
            "A3V_OVERWRITE_TEST": "1" if OVERWRITE_TEST else "0",
        })
        vuln_script = str(script_path_or_cwd(SCRIPT_VULNERABLE))
        rc = run_cmd([sys.executable, vuln_script], env, cwd=Path.cwd())
        log(f"MakeVulnerable returned {rc} for {rel}")
        time.sleep(0.2)

def remove_test_code_all(dest_root: Path) -> None:
    # run Remove_TestCode_3.py on every subdir in dest that contains A3_fixed.rs or A3_vulnerable.rs
    rem_script = script_path_or_cwd(SCRIPT_REMOVE)
    if not rem_script.exists():
        log("Remove_TestCode_3.py not found; skipping test-code removal.", "WARN")
        return
    for d in sorted([p for p in dest_root.rglob("*") if p.is_dir()]):
        names = {p.name for p in d.iterdir() if p.is_file()}
        if "A3_fixed.rs" in names or "A3_vulnerable.rs" in names:
            rc = run_cmd([sys.executable, str(rem_script), str(d)], os.environ.copy(), cwd=Path.cwd())
            log(f"Remove_TestCode for {d.relative_to(dest_root)} returned {rc}")
            time.sleep(0.05)

def run_oracles_all(dest_root: Path) -> None:
    # run Run_Oracles_3.py per subdir to validate and delete offenders
    orc_script = script_path_or_cwd(SCRIPT_ORACLES)
    if not orc_script.exists():
        log("Run_Oracles_3.py not found; skipping oracles run.", "WARN")
        return
    for d in sorted([p for p in dest_root.rglob("*") if p.is_dir()]):
        names = {p.name for p in d.iterdir() if p.is_file()}
        if not names.intersection({"A3_fixed.rs","A3_vulnerable.rs"}):
            continue
        rel = str(d.relative_to(dest_root))
        env = os.environ.copy()
        env.update({
            "A3_DST_ROOT": str(dest_root),
            "A3_CURRENT_SUBDIR": rel,
            "A3_DELETE_BAD": "1",  # enforce deletion of offending files
        })
        rc = run_cmd([sys.executable, str(orc_script)], env, cwd=Path.cwd())
        log(f"Run_Oracles for {rel} returned {rc}")
        time.sleep(0.05)

# ---------- Main orchestration loop ----------
def main():
    DEST.mkdir(parents=True, exist_ok=True)
    candidates = find_candidates(SOURCE)
    if not candidates:
        log("No candidate dirs found in SOURCE. Exiting.", "ERROR")
        return 2

    passes = 0
    while passes < MAX_PASSES:
        passes += 1
        log(f"*** Iteration {passes} start ***")
        fixed_count, vuln_count, test_count, total = count_outputs(DEST)
        log(f"Counts before iteration: fixed={fixed_count}, vuln={vuln_count}, demo_test={test_count}, total={total}")

        if total >= TARGET_TOTAL:
            log("Target total reached. Exiting with success.")
            return 0

        # Phase 1: generate fixed files up to TARGET_FIXED
        generate_fixed(candidates, SOURCE, DEST)

        # Phase 2: generate vulnerable files up to TARGET_VULN
        generate_vuln(candidates, SOURCE, DEST)

        # Phase 3: remove test code from outputs
        remove_test_code_all(DEST)

        # Phase 4: run oracles to validate and delete offenders
        run_oracles_all(DEST)

        # Recount and decide whether to continue
        fixed_count, vuln_count, test_count, total = count_outputs(DEST)
        log(f"Counts after pass: fixed={fixed_count}, vuln={vuln_count}, demo_test={test_count}, total={total}")

        if total >= TARGET_TOTAL:
            log("Target total reached after pass. Exiting with success.")
            return 0

        time.sleep(1.0)

    log(f"MAX_PASSES ({MAX_PASSES}) reached without meeting TARGET_TOTAL={TARGET_TOTAL}", "ERROR")
    return 1

if __name__ == '__main__':
    rc = main()
    sys.exit(rc)
