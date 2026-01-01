#!/usr/bin/env python3
"""
orchestrate_until_20.py
Runs, in order and repeatedly:
  1) DataGenerationFile.py
  2) Remove_TestCode.py
  3) Run_Oracles.py
until there are 20 'pair*' directories under the detected 'CWE-*' directory.
No hardcoded directory paths here; it discovers 'CWE-*' dynamically.
"""

import subprocess
import sys
import os
import re
from pathlib import Path
from typing import Optional, List

REQUIRED_COUNT = 20
SCRIPT_GENERATE = ["python3", "DataGenerationFile.py"]
SCRIPT_STRIP    = ["python3", "Remove_TestCode.py"]
SCRIPT_ORACLES  = ["python3", "Run_Oracles.py"]

PAIR_RE = re.compile(r"^pair\d+$")

def count_pairs(directory: Optional[Path]) -> int:
    if directory is None or not directory.exists():
        return 0
    return sum(1 for p in directory.iterdir() if p.is_dir() and PAIR_RE.match(p.name))

def find_cwe_dirs(start: Path) -> List[Path]:
    """Find all dirs named 'CWE-*' beneath start."""
    matches: List[Path] = []
    for dirpath, dirnames, _ in os.walk(start):
        d = Path(dirpath)
        if d.name == "CWE-662":
            matches.append(d)
    return matches

def pick_best_cwe(start: Path) -> Optional[Path]:
    """Pick the CWE-* dir with the most pair* subdirs (if any)."""
    candidates = find_cwe_dirs(start)
    if not candidates:
        return None
    return max(candidates, key=count_pairs)

def run_cmd(cmd) -> int:
    print(f"\n=== RUN: {' '.join(str(x) for x in cmd)} ===")
    proc = subprocess.run(cmd, text=True)
    print(f"[status] return code: {proc.returncode}")
    return proc.returncode

def main():
    iteration = 0
    last_count = -1

    while True:
        iteration += 1
        target_dir = pick_best_cwe(Path.cwd())
        current = count_pairs(target_dir)
        where = str(target_dir) if target_dir else "<not found yet>"
        print(f"\n---------------- Iteration {iteration} ----------------")
        print(f"[info] CWE-662 dir: {where}")
        print(f"[info] Current pir directories: {current}/{REQUIRED_COUNT}")

        if current >= REQUIRED_COUNT:
            print(f"\n[done] Reached required count ({current} >= {REQUIRED_COUNT}). Exiting.")
            break

        # 1) Generate (creates/updates pairs)
        rc = run_cmd(SCRIPT_GENERATE)
        if rc != 0:
            print(f"[error] DataGenerationFile.py failed (rc={rc}). Will continue loop and retry.")

        # 2) Strip any test code from fixed.rs/vulnerable.rs
        rc = run_cmd(SCRIPT_STRIP)
        if rc != 0:
            print(f"[error] Remove_TestCode.py failed (rc={rc}). Will continue loop and retry.")

        # 3) Compile & enforce oracle expectations (may delete bad dirs)
        rc = run_cmd(SCRIPT_ORACLES)
        if rc != 0:
            print(f"[warn] Run_Oracles.py reported deletions/failures (rc={rc}). Will regenerate in next iteration.")

        # Re-detect target dir (could have been created this round)
        target_dir = pick_best_cwe(Path.cwd())
        new_count = count_pairs(target_dir)
        where = str(target_dir) if target_dir else "<not found yet>"
        print(f"[info] End of iteration {iteration}: {new_count}/{REQUIRED_COUNT} pair directories present in {where}")

        if new_count == last_count:
            print("[note] No change this iteration; continuing until target is reached.")
        last_count = new_count

    final_target = pick_best_cwe(Path.cwd())
    final_count = count_pairs(final_target)
    print(f"\n========== SUMMARY ==========")
    print(f"CWE 662 dir: {str(final_target) if final_target else '<not found>'}")
    print(f"Final pair directories: {final_count}/{REQUIRED_COUNT}")
    print("[exit] success")
    sys.exit(0)

if __name__ == "__main__":
    main()
