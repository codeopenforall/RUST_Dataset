
#!/usr/bin/env python3
"""
orchestrate_qwen14b.py
Run the pipeline until we have a TOTAL of 60 files (20 pairs × 3 files) across A1_fixed.rs, A1_vulnerable.rs, demo_test.rs.

Changes in this version:
- Default model switched to: Qwen/Qwen2.5-Coder-14B-Instruct
- Keeps existing logic to: generate vulnerable + fixed → ensure tests → clean → run oracles (optionally delete offenders)
- Counts total files, not directories. Stops when total >= target (default 60).

Suggested environment (choose CUDA wheels matching your system):
    pip install -U "transformers>=4.44.0" "accelerate>=0.34.0" "torch>=2.1"
    # Optional (to avoid accidental vision deps):
    pip install --no-deps -U qwen-vl-utils  # not required for plain coder

Assumes helper scripts are available (same dir or on PATH):
    A1.MakeVulnerable.py, A1.MakeFixed.py, Remove_TestCode_1.py, Run_Oracles_1.py
"""

import argparse
import os
import re
import shlex
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Optional

PAIR_RE = re.compile(r"^pair\d+$")

# -------- Defaults (edit if needed) --------
DEFAULT_INPUT_ROOT  = "/home/ikqnm/PycharmProjects/PythonProject/DataGenerated_pairs/"
DEFAULT_OUTPUT_ROOT = "/home/ikqnm/PycharmProjects/PythonProject/deepseek/DataAugmented_pairs_new/"
TARGET_CWE          = "CWE-190"
DEFAULT_MODEL       = "Qwen/Qwen2.5-Coder-14B-Instruct"
DEFAULT_REQUIRED_FILES = 60  # We want a total of 60 files (20 pairs * 3 files)

# Helper scripts (assumed to be on PATH or in the same directory)
SCRIPT_GEN_VULN = "A1.MakeVulnerable.py"
SCRIPT_GEN_FIX  = "A1.MakeFixed.py"
SCRIPT_CLEAN    = "Remove_TestCode_1.py"
SCRIPT_TEST     = "Run_Oracles_1.py"


def list_pairs(dir_path: Path):
    if not dir_path.exists():
        return []
    try:
        return [p for p in dir_path.iterdir() if p.is_dir() and PAIR_RE.match(p.name)]
    except Exception:
        return []


def count_total_files(out_cwe: Path) -> int:
    """
    Count total number of target files in OUTPUT CWE dir:
      - A1_fixed.rs
      - A1_vulnerable.rs
      - demo_test.rs
    across all pair* subdirectories.
    """
    total = 0
    for p in list_pairs(out_cwe):
        for name in ("A1_fixed.rs", "A1_vulnerable.rs", "demo_test.rs"):
            if (p / name).exists():
                total += 1
    return total


def run_cmd(cmd, cwd: Optional[Path] = None) -> int:
    if isinstance(cmd, str):
        cmd = shlex.split(cmd)
    print(f"\n=== RUN: {' '.join(str(x) for x in cmd)} ===")
    proc = subprocess.run(cmd, text=True, cwd=cwd)
    print(f"[status] return code: {proc.returncode}")
    return proc.returncode


def ensure_script_exists(name: str):
    # Look relative to current file, then PATH
    here = Path(__file__).resolve().parent
    local = here / name
    if local.exists():
        return str(local)
    # fallback: let the shell/OS resolve via PATH
    return name


def copy_missing_tests(in_cwe: Path, out_cwe: Path) -> int:
    """
    Ensure each output pair directory has demo_test.rs by copying from input.
    Accepts any of (demo_test.rs, oracle.rs, orcale.rs) in the input pair.
    Returns the number of tests copied.
    """
    copied = 0
    for p in list_pairs(in_cwe):
        # pick source test in priority order
        src_test = None
        for name in ("demo_test.rs", "oracle.rs", "orcale.rs"):
            cand = p / name
            if cand.exists():
                src_test = cand
                break
        if not src_test:
            continue

        dst_dir = out_cwe / p.name
        dst_dir.mkdir(parents=True, exist_ok=True)
        dst = dst_dir / "demo_test.rs"
        if not dst.exists():
            try:
                shutil.copy2(src_test, dst)
                print(f"[copied test] {src_test} -> {dst}")
                copied += 1
            except Exception as e:
                print(f"[warn] failed to copy test for {p.name}: {e}")
    return copied


def main():
    ap = argparse.ArgumentParser(
        description="Orchestrate gen(A1_vuln & A1_fixed) -> ensure tests -> clean -> test until total files == 60."
    )
    ap.add_argument("--input-root",  default=DEFAULT_INPUT_ROOT,
                    help="Top-level input root (contains CWE/pair*)")
    ap.add_argument("--output-root", default=DEFAULT_OUTPUT_ROOT,
                    help="Top-level output root (will mirror CWE)")
    ap.add_argument("--required-files", type=int, default=DEFAULT_REQUIRED_FILES,
                    help="Total target files across pair* (default: 60 = 20 pairs * 3 files)")
    ap.add_argument("--model",       default=DEFAULT_MODEL,
                    help="HF model id for generation (default: Qwen/Qwen2.5-Coder-14B-Instruct)")
    ap.add_argument("--delete-bad",  action="store_true",
                    help="In the test step, delete offending A1_*.rs files that violate the contract")
    ap.add_argument("--force-run",   action="store_true",
                    help="Run one full iteration even if target already met")
    args = ap.parse_args()

    in_root  = Path(args.input_root).expanduser().resolve()
    out_root = Path(args.output_root).expanduser().resolve()

    in_cwe  = in_root  / TARGET_CWE
    out_cwe = out_root / TARGET_CWE
    out_cwe.mkdir(parents=True, exist_ok=True)

    # Resolve scripts (local file or PATH)
    gen_vuln = ensure_script_exists(SCRIPT_GEN_VULN)
    gen_fix  = ensure_script_exists(SCRIPT_GEN_FIX)
    clean    = ensure_script_exists(SCRIPT_CLEAN)
    test     = ensure_script_exists(SCRIPT_TEST)

    # Confirm we have some input pair dirs
    input_pairs = list_pairs(in_cwe)
    if not input_pairs:
        print(f"[error] No pair* directories found under: {in_cwe}")
        sys.exit(2)

    # If the required files exceed what is theoretically possible, cap it:
    # each input pair contributes up to 3 files (A1_fixed, A1_vulnerable, demo_test).
    max_possible_files = 3 * len(input_pairs)
    required_files = args.required_files
    if max_possible_files < required_files:
        print(f"[warn] Only {len(input_pairs)} input pair* dirs found; "
              f"maximum possible files = {max_possible_files}. "
              f"Capping target from {required_files} -> {max_possible_files}")
        required_files = max_possible_files

    iteration = 0
    ran_once = False

    while True:
        # Refresh state each loop
        input_pairs = list_pairs(in_cwe)
        total_files = count_total_files(out_cwe)

        iteration += 1
        print(f"\n---------------- Iteration {iteration} ----------------")
        print(f"[info] INPUT  CWE dir: {in_cwe}")
        print(f"[info] OUTPUT CWE dir: {out_cwe}")
        print(f"[info] Input pair* dirs : {len(input_pairs)}")
        print(f"[info] Total files (A1_fixed, A1_vulnerable, demo_test) : {total_files}/{required_files}")

        if (total_files >= required_files) and (not args.force_run or ran_once):
            print(f"\n[done] Target met (files={total_files} >= required={required_files}). Exiting.")
            break

        # 0) Ensure tests exist in output for every pair
        _ = copy_missing_tests(in_cwe, out_cwe)

        # 1) Generate A1_vulnerable.rs where missing
        rc = run_cmd([
            "python3", gen_vuln,
            "--input",  str(in_cwe),
            "--output", str(out_cwe),
            "--model",  args.model,
            "--only-missing",
        ])
        if rc != 0:
            print(f"[warn] vulnerable generation step returned non-zero (rc={rc}). Continuing.")

        # 2) Generate A1_fixed.rs where missing
        rc = run_cmd([
            "python3", gen_fix,
            "--input",  str(in_cwe),
            "--output", str(out_cwe),
            "--model",  args.model,
            "--only-missing",
        ])
        if rc != 0:
            print(f"[warn] fixed generation step returned non-zero (rc={rc}). Continuing.")

        # 2.5) Ensure tests again, in case we just created a new out pair dir
        _ = copy_missing_tests(in_cwe, out_cwe)

        # 3) Clean (optional post-processing of produced sources)
        rc = run_cmd(["python3", clean, "--root", str(out_cwe)])
        if rc != 0:
            print(f"[warn] cleaning step returned non-zero (rc={rc}). Continuing.")

        # 4) Test + (optionally) delete offending files
        test_cmd = ["python3", test, "--root", str(out_cwe)]
        if args.delete_bad:
            test_cmd.append("--delete-bad")
        rc = run_cmd(test_cmd)
        if rc != 0:
            print(f"[warn] test step returned non-zero (rc={rc}). See logs above.")

        ran_once = True

    final_files = count_total_files(out_cwe)
    print("\n========== SUMMARY ==========")
    print(f"Input pair* dirs        : {len(list_pairs(in_cwe))} under {in_cwe}")
    print(f"Total files present     : {final_files} under {out_cwe}")
    print(f"Target required (files) : {required_files}")
    print("[exit] success")
    sys.exit(0)


if __name__ == "__main__":
    main()