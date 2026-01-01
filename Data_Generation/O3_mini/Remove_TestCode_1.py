#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
orchestrate_until_20_3.py  (updated to loop until total files >= 60)

Calls, in this order for each candidate subdir:
  1) A3.MakeFixed.py       -> creates A3_fixed.rs and mirrors demo_test.rs
  2) A3.MakeVulnerable.py  -> creates A3_vulnerable.rs and mirrors demo_test.rs
  3) Run_Oracles_3.py      -> validates & (optionally) deletes offenders via env
  4) Remove_TestCode_3.py  -> strips test code from generated sources

Repeats passes until (count(A3_fixed.rs) + count(A3_vulnerable.rs) + count(demo_test.rs)) >= TARGET_TOTAL (=60).

Edit SOURCE/DEST below if needed.
"""

import os
import sys
import time
import subprocess
from pathlib import Path
from typing import List, Tuple
from datetime import datetime

# ---------- Fixed paths ----------
SOURCE = Path("/home/ikqnm/PycharmProjects/PythonProject/O3_mini/Cleaned_DataGenerated_pairs_CC/CWE-020").resolve()
DEST   = Path("/home/ikqnm/PycharmProjects/PythonProject/deepseek/DataAugmented_pairs_new_A3/CWE-020").resolve()

# ---------- Targets & controls ----------
TARGET_TOTAL    = int(os.environ.get("TARGET_TOTAL", "60"))  # fixed + vulnerable + demo_test
MAX_PASSES      = int(os.environ.get("MAX_PASSES", "200"))   # safety cap
ONLY_MISSING    = True                                       # skip re-gen if files exist (unless deleted by oracles)
OVERWRITE_TEST  = True                                       # mirror demo_test.rs into DEST
SLEEP_BETWEEN   = float(os.environ.get("SLEEP_BETWEEN", "1.5"))

# ---------- Script names ----------
SCRIPT_FIXED      = Path("A3.MakeFixed.py")
SCRIPT_VULNERABLE = Path("A3.MakeVulnerable.py")
SCRIPT_ORACLES    = Path("Run_Oracles_3.py")
SCRIPT_REMOVE     = Path("Remove_TestCode_3.py")

# ---------- Logging ----------
def log(msg: str, level: str="INFO"):
    ts = datetime.now().strftime("%H:%M:%S")
    print(f"{ts} [{level}] {msg}")

def warn(msg: str): log(msg, "WARN")
def err(msg: str):  log(msg, "ERROR")

# ---------- Discovery ----------
def dir_has_required(d: Path) -> bool:
    names = {p.name for p in d.iterdir() if p.is_file()}
    return ("fixed_diff.rs" in names) and ("demo_test.rs" in names)

def find_candidate_dirs(root: Path) -> List[Path]:
    out: List[Path] = []
    if not root.exists():
        return out
    for d in sorted([p for p in root.rglob("*") if p.is_dir()]):
        try:
            if dir_has_required(d):
                out.append(d)
        except Exception:
            continue
    return out

# ---------- Counters ----------
def count_outputs(dest_root: Path) -> Tuple[int, int, int, int]:
    fixed = vuln = tests = total = 0
    if not dest_root.exists():
        return 0,0,0,0
    for d in sorted([p for p in dest_root.rglob("*") if p.is_dir()]):
        names = {p.name for p in d.iterdir() if p.is_file()}
        if "A3_fixed.rs" in names:       fixed += 1
        if "A3_vulnerable.rs" in names:  vuln += 1
        if "demo_test.rs" in names:      tests += 1
    total = fixed + vuln + tests
    return fixed, vuln, tests, total

# ---------- Helpers ----------
def rel_from_root(p: Path, root: Path) -> str:
    try:
        return str(p.relative_to(root))
    except Exception:
        return p.name

def run_cmd(cmd: list, env: dict, cwd: Path) -> int:
    log(f"RUN: {' '.join(cmd)}  (cwd={cwd})")
    try:
        res = subprocess.run(cmd, cwd=str(cwd), env=env, capture_output=True, text=True, timeout=3600)
    except Exception as e:
        err(f"Command failed: {e}")
        return 127
    if res.stdout:
        log("stdout:\n" + res.stdout.strip(), "INFO")
    if res.stderr:
        log("stderr:\n" + res.stderr.strip(), "ERROR")
    return res.returncode

def ensure_script_present(script: Path) -> bool:
    return script.is_file() or (Path.cwd()/script).is_file()

# ---------- Main ----------
def main(argv: list) -> int:
    log(f"SOURCE: {SOURCE}")
    log(f"DEST  : {DEST}")
    DEST.mkdir(parents=True, exist_ok=True)

    if not (ensure_script_present(SCRIPT_FIXED) and ensure_script_present(SCRIPT_VULNERABLE)):
        warn("Missing A3.MakeFixed.py or A3.MakeVulnerable.py near this script (or on PATH).")

    passes = 0
    while passes < MAX_PASSES:
        passes += 1
        f_cnt, v_cnt, t_cnt, tot = count_outputs(DEST)
        log(f"=== Pass {passes} ===  counts -> fixed={f_cnt} vuln={v_cnt} demo_test={t_cnt} total={tot} / target={TARGET_TOTAL}")
        if tot >= TARGET_TOTAL:
            log("Target reached. Exiting.")
            return 0

        cands = find_candidate_dirs(SOURCE)
        if not cands:
            warn("No candidate subdirectories found (need fixed_diff.rs + demo_test.rs).")
            time.sleep(SLEEP_BETWEEN)
            continue

        for src_dir in cands:
            # Re-check total before processing each candidate
            f_cnt, v_cnt, t_cnt, tot = count_outputs(DEST)
            if tot >= TARGET_TOTAL:
                break

            rel = rel_from_root(src_dir, SOURCE)
            out_dir = DEST / rel
            out_dir.mkdir(parents=True, exist_ok=True)

            log(f"--- Candidate: {rel} ---")

            # Step 1: MakeFixed
            need_fixed = (not ONLY_MISSING) or not (out_dir / "A3_fixed.rs").exists()
            if need_fixed:
                env_fixed = os.environ.copy()
                env_fixed.update({
                    "A3_SRC_ROOT": str(SOURCE),
                    "A3_DST_ROOT": str(DEST),
                    "A3_CURRENT_SUBDIR": rel,
                    "A3_ONLY_MISSING": "1" if ONLY_MISSING else "0",
                    "A3_OVERWRITE_TEST": "1" if OVERWRITE_TEST else "0",
                })
                fixed_path = SCRIPT_FIXED if SCRIPT_FIXED.is_file() else (Path.cwd()/SCRIPT_FIXED)
                rc1 = run_cmd([sys.executable, str(fixed_path)], env_fixed, cwd=Path.cwd())
                log(f"A3.MakeFixed.py rc={rc1}")

            # Step 2: MakeVulnerable
            need_vuln = (not ONLY_MISSING) or not (out_dir / "A3_vulnerable.rs").exists()
            if need_vuln:
                env_v = os.environ.copy()
                env_v.update({
                    "A3V_SRC_ROOT": str(SOURCE),
                    "A3V_DST_ROOT": str(DEST),
                    "A3V_CURRENT_SUBDIR": rel,
                    "A3V_ONLY_MISSING": "1" if ONLY_MISSING else "0",
                    "A3V_OVERWRITE_TEST": "1" if OVERWRITE_TEST else "0",
                })
                vuln_path = SCRIPT_VULNERABLE if SCRIPT_VULNERABLE.is_file() else (Path.cwd()/SCRIPT_VULNERABLE)
                rc2 = run_cmd([sys.executable, str(vuln_path)], env_v, cwd=Path.cwd())
                log(f"A3.MakeVulnerable.py rc={rc2}")

            # Step 3: Run oracles (env-driven; hard-delete offenders if script supports it)
            if ensure_script_present(SCRIPT_ORACLES):
                env_o = os.environ.copy()
                env_o.update({
                    "A3_DST_ROOT": str(DEST),
                    "A3_CURRENT_SUBDIR": rel,
                    "A3_DELETE_BAD": "1",  # if supported, delete offenders
                })
                orc_path = SCRIPT_ORACLES if SCRIPT_ORACLES.is_file() else (Path.cwd()/SCRIPT_ORACLES)
                rc3 = run_cmd([sys.executable, str(orc_path)], env_o, cwd=Path.cwd())
                log(f"Run_Oracles_3.py rc={rc3}")
            else:
                warn("Run_Oracles_3.py not found; skipping test step.")

            # Step 4: Remove test code (takes target dir as CLI arg)
            if ensure_script_present(SCRIPT_REMOVE):
                rem_path = SCRIPT_REMOVE if SCRIPT_REMOVE.is_file() else (Path.cwd()/SCRIPT_REMOVE)
                rc4 = run_cmd([sys.executable, str(rem_path), str(out_dir)], os.environ.copy(), cwd=Path.cwd())
                log(f"Remove_TestCode_3.py rc={rc4}")
            else:
                warn("Remove_TestCode_3.py not found; skipping cleanup.")

            # Small delay between candidates
            time.sleep(0.2)

        # End for candidates
        # Short pause before next pass
        time.sleep(SLEEP_BETWEEN)

    err(f"MAX_PASSES reached ({MAX_PASSES}) without achieving TARGET_TOTAL={TARGET_TOTAL}.")
    return 1

if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))