#!/usr/bin/env python3
"""
orchestrate_A1_pipeline.py
End-to-end pipeline for Rust pair generation and validation (A1 policy).

Sequence:
  0) (optional) Rename all oracle.rs -> demo_test.rs in INPUT root
  1) Data augmentation (DeepSeek)      : A1.DataAugmentation.py
  2) Data cleaning (strip comments)    : Remove_TestCode.py
  3) Run unit tests & enforce contract : Run_Oracles.py

Defaults are set for your environment; override via CLI flags.

Usage examples:
  python orchestrate_A1_pipeline.py
  python orchestrate_A1_pipeline.py --skip-rename
  python orchestrate_A1_pipeline.py --input /path/in --output /path/out --model deepseek-ai/deepseek-coder-6.7b-instruct --delete-bad
"""

import subprocess
import sys
from pathlib import Path
import argparse

# ---------- Defaults matching your setup ----------
DEFAULT_INPUT  = "/home/ikqnm/PycharmProjects/PythonProject/O3_mini/DataGenerated_pairs_new/"
DEFAULT_OUTPUT = "/home/ikqnm/PycharmProjects/PythonProject/deepseek/DataAugmented_pairs_new/"
DEFAULT_MODEL  = "deepseek-ai/deepseek-coder-6.7b-instruct"

# The helper scripts should be accessible on PATH or by relative path
SCRIPT_AUGMENT = "A1.DataAugmentation.py"
SCRIPT_CLEAN   = "Remove_TestCode.py"
SCRIPT_TEST    = "Run_Oracles.py"
SCRIPT_RENAME  = "rename_oracle_to_demo_test.py"  # your file 4

def run_cmd(cmd, cwd: Path | None = None) -> int:
    print(f"\n=== RUN: {' '.join(str(x) for x in cmd)} ===")
    proc = subprocess.run(cmd, text=True, cwd=cwd)
    print(f"[status] return code: {proc.returncode}")
    return proc.returncode

def main():
    ap = argparse.ArgumentParser(description="Orchestrate A1 data augmentation -> cleaning -> testing.")
    ap.add_argument("--input",  default=DEFAULT_INPUT,  help="Input root containing CWE-* folders")
    ap.add_argument("--output", default=DEFAULT_OUTPUT, help="Output root for augmented pairs")
    ap.add_argument("--model",  default=DEFAULT_MODEL,  help="Hugging Face model id for augmentation")
    ap.add_argument("--skip-rename", action="store_true", help="Skip renaming oracle.rs -> demo_test.rs in input root")
    ap.add_argument("--delete-bad", action="store_true", help="Delete dirs that fail the contract in test step")
    args = ap.parse_args()

    in_root = str(Path(args.input).expanduser().resolve())
    out_root = str(Path(args.output).expanduser().resolve())

    if rc != 0:
        print("[warn] augmentation returned non-zero; continuing")

    # 2) data cleaning (strip comments + drop empty lines) on generated A1 files
    rc = run_cmd(["python3", SCRIPT_CLEAN, "--root", out_root])
    if rc != 0:
        print("[warn] cleaning returned non-zero; continuing")

    # 3) run tests & enforce contract
    cmd = ["python3", SCRIPT_TEST, "--root", out_root]
    if args.delete_bad:
        cmd.append("--delete-bad")
    rc = run_cmd(cmd)
    if rc != 0:
        print("[warn] tests returned non-zero; see logs above")

    print("\n[done] Pipeline execution complete.")

if __name__ == "__main__":
    main()
