#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
orchestrate_until_20_3.py — enhanced orchestrator (looping)

Saves: run until TARGET_FIXED (20) and TARGET_VULN (20) are present in DEST.
Uses:
  - A3.MakeFixed.py
  - A3.MakeVulnerable.py
  - Run_Oracles_3.py     (called with A3_DELETE_BAD=1 to remove offenders)
  - Remove_TestCode_3.py

Configure by editing constants at top or setting env vars:
  TARGET_FIXED (default 20)
  TARGET_VULN  (default 20)
  MAX_PASSES   (safety cap, default 100)

Paths (defaults hard-coded for your environment):
  SOURCE = /home/ikqnm/PycharmProjects/PythonProject/O3_mini/Cleaned_DataGenerated_pairs_CC/CWE-X
  DEST   = /home/ikqnm/PycharmProjects/PythonProject/deepseek/DataAugmented_pairs_new_A3/CWE-X
"""

import os, sys, time, subprocess
from pathlib import Path
from datetime import datetime
from typing import List, Tuple

# ---------------- Configuration ----------------
SOURCE = Path("/home/ikqnm/PycharmProjects/PythonProject/O3_mini/Cleaned_DataGenerated_pairs_CC/CWE-020").resolve()
DEST   = Path("/home/ikqnm/PycharmProjects/PythonProject/deepseek/DataAugmented_pairs_new_A3/CWE-020").resolve()

TARGET_FIXED = int(os.environ.get("TARGET_FIXED", "20"))
TARGET_VULN  = int(os.environ.get("TARGET_VULN",  "20"))
MAX_PASSES   = int(os.environ.get("MAX_PASSES",  "100"))  # safety cap
ONLY_MISSING = True   # skip generating if output exists (unless deleted by oracles)
OVERWRITE_TEST = True

# Script file names (expected next to this orchestrator or on PATH)
SCRIPT_FIXED      = Path("A3.MakeFixed.py")
SCRIPT_VULNERABLE = Path("A3.MakeVulnerable.py")
SCRIPT_ORACLES    = Path("Run_Oracles_3.py")
SCRIPT_REMOVE     = Path("Remove_TestCode_3.py")

SLEEP_BETWEEN_PASSES = 2  # seconds between passes

# ---------------- Logging ----------------
def log(msg: str, level: str="INFO"):
    ts = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    print(f"{ts} [{level}] {msg}")

# ---------------- Helpers ----------------
def find_candidates(src_root: Path) -> List[Path]:
    """Candidates: directories that contain fixed_diff.rs and demo_test.rs"""
    out = []
    if not src_root.exists():
        return out
    for d in sorted([p for p in src_root.rglob("*") if p.is_dir()]):
        names = {p.name for p in d.iterdir() if p.is_file()}
        if "fixed_diff.rs" in names and "demo_test.rs" in names:
            out.append(d)
    return out

def count_outputs(dest_root: Path) -> Tuple[int, int]:
    """Count A3_fixed.rs and A3_vulnerable.rs under DEST."""
    fixed_count = 0
    vuln_count = 0
    if not dest_root.exists():
        return (0,0)
    for d in sorted([p for p in dest_root.rglob("*") if p.is_dir()]):
        names = {p.name for p in d.iterdir() if p.is_file()}
        if "A3_fixed.rs" in names:
            fixed_count += 1
        if "A3_vulnerable.rs" in names:
            vuln_count += 1
    return fixed_count, vuln_count

def run_cmd(cmd: List[str], env: dict, cwd: Path) -> int:
    """Run a command (blocking) and log stdout/stderr."""
    log(f"RUN: {' '.join(cmd)}  (cwd={cwd})")
    try:
        res = subprocess.run(cmd, cwd=str(cwd), env=env, capture_output=True, text=True, timeout=3600)
    except Exception as e:
        log(f"Command failed to run: {e}", "ERROR")
        return 127
    if res.stdout:
        log("stdout:\n" + res.stdout.strip(), "INFO")
    if res.stderr:
        log("stderr:\n" + res.stderr.strip(), "ERROR")
    return res.returncode

def ensure_script_available(script_path: Path) -> bool:
    """Check if script file exists next to orchestrator or is on PATH (best-effort)."""
    if script_path.is_file():
        return True
    if (Path.cwd()/script_path).is_file():
        return True
    # Not on disk; might be on PATH — we return True but runtime invocation may fail.
    return True

# ---------------- Main loop ----------------
def main_loop() -> int:
    # sanity checks: require the generator scripts to be found next to this file (best-effort)
    if not ensure_script_available(SCRIPT_FIXED) or not ensure_script_available(SCRIPT_VULNERABLE):
        log("Generator scripts A3.MakeFixed.py / A3.MakeVulnerable.py should be placed next to this orchestrator or on PATH.", "WARN")

    DEST.mkdir(parents=True, exist_ok=True)

    passes = 0
    while passes < MAX_PASSES:
        passes += 1
        log(f"=== Pass {passes} start ===")
        fixed_count, vuln_count = count_outputs(DEST)
        log(f"Current: fixed={fixed_count} vuln={vuln_count} (targets fixed={TARGET_FIXED}, vuln={TARGET_VULN})")

        if fixed_count >= TARGET_FIXED and vuln_count >= TARGET_VULN:
            log("Targets achieved — exiting with success.")
            return 0

        candidates = find_candidates(SOURCE)
        if not candidates:
            log("No candidate subdirectories found in SOURCE (need fixed_diff.rs + demo_test.rs).", "ERROR")
            return 2

        for src_dir in candidates:
            if fixed_count >= TARGET_FIXED and vuln_count >= TARGET_VULN:
                break

            rel = str(src_dir.relative_to(SOURCE))
            out_dir = DEST / rel
            out_dir.mkdir(parents=True, exist_ok=True)

            log(f"-- Candidate: {rel} --")

            # 1) Generate A3_fixed.rs if needed
            if fixed_count < TARGET_FIXED and (not ONLY_MISSING or not (out_dir/"A3_fixed.rs").exists()):
                env_fixed = os.environ.copy()
                env_fixed.update({
                    "A3_SRC_ROOT": str(SOURCE),
                    "A3_DST_ROOT": str(DEST),
                    "A3_CURRENT_SUBDIR": rel,
                    "A3_ONLY_MISSING": "1" if ONLY_MISSING else "0",
                    "A3_OVERWRITE_TEST": "1" if OVERWRITE_TEST else "0",
                })
                fixed_script = str(SCRIPT_FIXED if SCRIPT_FIXED.is_file() else (Path.cwd()/SCRIPT_FIXED))
                rc = run_cmd([sys.executable, fixed_script], env_fixed, cwd=Path.cwd())
                if rc == 0 and (out_dir/"A3_fixed.rs").exists():
                    fixed_count += 1
                    log(f"Created A3_fixed.rs ({fixed_count}/{TARGET_FIXED})")
                else:
                    log(f"A3_fixed generation returned rc={rc}", "WARN")

            # 2) Generate A3_vulnerable.rs if needed
            if vuln_count < TARGET_VULN and (not ONLY_MISSING or not (out_dir/"A3_vulnerable.rs").exists()):
                env_vuln = os.environ.copy()
                env_vuln.update({
                    "A3V_SRC_ROOT": str(SOURCE),
                    "A3V_DST_ROOT": str(DEST),
                    "A3V_CURRENT_SUBDIR": rel,
                    "A3V_ONLY_MISSING": "1" if ONLY_MISSING else "0",
                    "A3V_OVERWRITE_TEST": "1" if OVERWRITE_TEST else "0",
                })
                vuln_script = str(SCRIPT_VULNERABLE if SCRIPT_VULNERABLE.is_file() else (Path.cwd()/SCRIPT_VULNERABLE))
                rc = run_cmd([sys.executable, vuln_script], env_vuln, cwd=Path.cwd())
                if rc == 0 and (out_dir/"A3_vulnerable.rs").exists():
                    vuln_count += 1
                    log(f"Created A3_vulnerable.rs ({vuln_count}/{TARGET_VULN})")
                else:
                    log(f"A3_vulnerable generation returned rc={rc}", "WARN")

            # 3) Run oracles to validate and enforce deletion of offenders
            if (out_dir/"A3_fixed.rs").exists() or (out_dir/"A3_vulnerable.rs").exists():
                if ensure_script_available(SCRIPT_ORACLES):
                    env_o = os.environ.copy()
                    env_o.update({
                        "A3_DST_ROOT": str(DEST),
                        "A3_CURRENT_SUBDIR": rel,
                        "A3_DELETE_BAD": "1",   # enforce removal on failing contract
                    })
                    oracles_script = str(SCRIPT_ORACLES if SCRIPT_ORACLES.is_file() else (Path.cwd()/SCRIPT_ORACLES))
                    rc = run_cmd([sys.executable, oracles_script], env_o, cwd=Path.cwd())
                    log(f"Run_Oracles returned rc={rc} for {rel}")
                    # if oracle removed files, adjust counts
                    if not (out_dir/"A3_fixed.rs").exists() and fixed_count>0:
                        fixed_count -= 1
                        log("A3_fixed.rs removed by oracle (failed contract).")
                    if not (out_dir/"A3_vulnerable.rs").exists() and vuln_count>0:
                        vuln_count -= 1
                        log("A3_vulnerable.rs removed by oracle (failed contract).")
                else:
                    log("Run_Oracles_3.py not found; skipping validation", "WARN")

            # 4) Strip test code from surviving files
            if ensure_script_available(SCRIPT_REMOVE):
                remove_script = str(SCRIPT_REMOVE if SCRIPT_REMOVE.is_file() else (Path.cwd()/SCRIPT_REMOVE))
                rc = run_cmd([sys.executable, remove_script, str(out_dir)], os.environ.copy(), cwd=Path.cwd())
                log(f"Remove_TestCode returned rc={rc} for {rel}")
            else:
                log("Remove_TestCode_3.py not found; skipping test removal", "WARN")

            # brief pause to avoid hammering
            time.sleep(0.5)

        # end for candidates

        fixed_count, vuln_count = count_outputs(DEST)
        log(f"End of pass {passes}: fixed={fixed_count} vuln={vuln_count}")

        if fixed_count >= TARGET_FIXED and vuln_count >= TARGET_VULN:
            log("Goal reached after pass {0}".format(passes))
            return 0

        time.sleep(SLEEP_BETWEEN_PASSES)

    log(f"Reached MAX_PASSES={MAX_PASSES} without hitting targets.", "ERROR")
    return 1

if __name__ == "__main__":
    rc = main_loop()
    sys.exit(rc)
