#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
orchestrate_until_20_2.py (phased A2 pipeline)
Centralized paths (edit only the two constants below). This orchestrator runs in phases:
  Phase A: generate up to 20 missing A2_vulnerable.rs files
  Phase B: generate up to 20 missing A2_fixed.rs files
  Phase C: remove test code (cleanup)
  Phase D: run oracles for all pairs (log to CSV)

Repeat phases until total generated files (A2_fixed.rs + A2_vulnerable.rs + demo_test.rs)
reaches REQUIRED_FILES (default 60).

Child scripts must be available in same directory or on PATH:
  - A2.MakeVulnerable.py   (env-only)
  - A2.MakeFixed.py        (env-only)
  - Remove_TestCode_2.py   (env-only)
  - Run_Oracles_2.py       (env-only)
"""

import os
import re
import shutil
import subprocess
import sys
from pathlib import Path
from typing import List, Optional

# ================== EDIT THESE TWO PATHS ONLY ==================
SRC_ROOT = "/home/ikqnm/PycharmProjects/PythonProject/O3_mini/Cleaned_DataGenerated_pairs_new/CWE-190"
DST_ROOT = "/home/ikqnm/PycharmProjects/PythonProject/deepseek/DataAugmented_pairs_new_A2/CWE-190"
# ===============================================================

SUMMARY_CSV = "CWE-190_A2_summary.csv"

# Stop condition: total files across pairs (A2_fixed.rs, A2_vulnerable.rs, demo_test.rs) >= REQUIRED_FILES
REQUIRED_FILES = 60   # 20 pairs × 3 files
PHASE_BATCH = 20      # number of items to create per phase (vuln/fix)

# Names of helper scripts (local file or PATH-resolved)
SCRIPT_GEN_VULN = "A2.MakeVulnerable.py"
SCRIPT_GEN_FIX  = "A2.MakeFixed.py"
SCRIPT_CLEAN    = "Remove_TestCode_2.py"
SCRIPT_TEST     = "Run_Oracles_2.py"

PAIR_RE = re.compile(r"^pair\d+$")

def ensure_script(name: str) -> str:
    here = Path(__file__).resolve().parent
    local = here / name
    return str(local) if local.exists() else name

def list_pairs(dir_path: Path) -> List[Path]:
    if not dir_path.exists():
        return []
    try:
        return [p for p in dir_path.iterdir() if p.is_dir() and PAIR_RE.match(p.name)]
    except Exception:
        return []

def count_total_files(out_cwe: Path) -> int:
    total = 0
    for p in list_pairs(out_cwe):
        for name in ("A2_fixed.rs", "A2_vulnerable.rs", "demo_test.rs"):
            if (p / name).exists():
                total += 1
    return total

def pick_test_file(src_pair: Path) -> Optional[Path]:
    for name in ("demo_test.rs", "oracle.rs", "orcale.rs"):
        cand = src_pair / name
        if cand.exists():
            return cand
    return None

def copy_test_if_missing(src_pair: Path, dst_pair: Path) -> bool:
    dst_pair.mkdir(parents=True, exist_ok=True)
    dst_test = dst_pair / "demo_test.rs"
    if dst_test.exists():
        return False
    src_test = pick_test_file(src_pair)
    if not src_test:
        return False
    try:
        shutil.copy2(src_test, dst_test)
        print(f"[copied test] {src_test} -> {dst_test}")
        return True
    except Exception as e:
        print(f"[warn] failed to copy test for {src_pair.name}: {e}")
        return False

def run_script(script: str, env: dict) -> int:
    cmd = [sys.executable, ensure_script(script)]
    print(f"\n=== RUN: {' '.join(cmd)} (A2_CURRENT_SUBDIR={env.get('A2_CURRENT_SUBDIR','')}) ===")
    proc = subprocess.run(cmd, text=True, env=env)
    print(f"[status] return code: {proc.returncode}")
    return proc.returncode

def phase_generate(target_kind: str, src_root: Path, dst_root: Path, max_create: int, base_env: dict) -> int:
    created = 0
    pairs = list_pairs(src_root)
    for src_pair in pairs:
        if created >= max_create:
            break
        dst_pair = dst_root / src_pair.name
        dst_pair.mkdir(parents=True, exist_ok=True)
        copy_test_if_missing(src_pair, dst_pair)
        env = dict(base_env)
        env["A2_CURRENT_SUBDIR"] = src_pair.name
        if target_kind == "vulnerable":
            target_file = dst_pair / "A2_vulnerable.rs"
            script = SCRIPT_GEN_VULN
        else:
            target_file = dst_pair / "A2_fixed.rs"
            script = SCRIPT_GEN_FIX
        if target_file.exists():
            continue
        rc = run_script(script, env)
        if rc == 0 and target_file.exists():
            created += 1
        else:
            print(f"[warn] {script} returned {rc} for {src_pair.name}")
    return created

def main():
    in_cwe = Path(SRC_ROOT).expanduser().resolve()
    out_cwe = Path(DST_ROOT).expanduser().resolve()
    out_cwe.mkdir(parents=True, exist_ok=True)

    base_env = os.environ.copy()
    base_env["A2_SRC_ROOT"] = str(in_cwe)
    base_env["A2_DST_ROOT"] = str(out_cwe)
    base_env["A2_SUMMARY_CSV"] = SUMMARY_CSV

    pairs = list_pairs(in_cwe)
    if not pairs:
        print(f"[error] No pair* directories found under: {in_cwe}")
        sys.exit(2)

    max_possible = 3 * len(pairs)
    required = min(REQUIRED_FILES, max_possible)
    if required < REQUIRED_FILES:
        print(f"[warn] Only {len(pairs)} input pairs; capping REQUIRED_FILES from {REQUIRED_FILES} -> {required}")

    iteration = 0
    while True:
        iteration += 1
        total = count_total_files(out_cwe)
        print(f"\n---------------- Iteration {iteration} ----------------")
        print(f"[info] INPUT  CWE dir: {in_cwe}")
        print(f"[info] OUTPUT CWE dir: {out_cwe}")
        print(f"[info] Input pair* dirs : {len(list_pairs(in_cwe))}")
        print(f"[info] Total files (A2_fixed, A2_vulnerable, demo_test) : {total}/{required}")

        if total >= required:
            print(f"\n[done] Target met (files={total} >= required={required}). Exiting.")
            break

        need = required - total
        batch = min(PHASE_BATCH, need)
        print(f"[phase A] generate up to {batch} A2_vulnerable.rs")
        created_v = phase_generate("vulnerable", in_cwe, out_cwe, batch, base_env)
        print(f"[phase A] created {created_v} vulnerable files")

        total = count_total_files(out_cwe)
        if total >= required:
            continue

        need = required - total
        batch = min(PHASE_BATCH, need)
        print(f"[phase B] generate up to {batch} A2_fixed.rs")
        created_f = phase_generate("fixed", in_cwe, out_cwe, batch, base_env)
        print(f"[phase B] created {created_f} fixed files")

        print("[phase C] cleaning test artifacts in destination pairs")
        for src_pair in list_pairs(in_cwe):
            env = dict(base_env)
            env["A2_CURRENT_SUBDIR"] = src_pair.name
            run_script(SCRIPT_CLEAN, env)

        print("[phase D] running oracles for all destination pairs (will append to CSV)")
        for src_pair in list_pairs(in_cwe):
            env = dict(base_env)
            env["A2_CURRENT_SUBDIR"] = src_pair.name
            run_script(SCRIPT_TEST, env)

    final_total = count_total_files(out_cwe)
    print("\n========== SUMMARY ==========")
    print(f"Input pair* dirs        : {len(list_pairs(in_cwe))} under {in_cwe}")
    print(f"Total files present     : {final_total} under {out_cwe}")
    print(f"Target required (files) : {required}")
    print("[exit] success")
    sys.exit(0)

if __name__ == "__main__":
    main()