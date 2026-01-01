# RUST-VGA: Benchmark Dataset for Rust Vulnerability Detection
This repository contains the dataset used in the paper **"RUST-VGA: Leverage LLMs to Generate a Benchmark Dataset for Rust**".

RUST-VGA is a large-scale, executable benchmark for **binary vulnerability detection in Rust**, built to support training and evaluation of ML/LLM-based detectors.

- **8,480 executable Rust programs**
- **53 CWE categories**
- Paired **vulnerable** and **fixed** variants
- Each sample includes **tests** used for validation

## What’s in this repository

This repo contains:
- The full RUST-VGA dataset (generated + augmented)
- Generation and augmentation code
- Validation scripts (compile + test; optional ASan checks)

---

## Dataset overview

Each sample is a Rust mini-project with:
- `vulnerable.rs` and `fixed.rs` (paired variants)
- `demo_test.rs` (test that should **fail** on vulnerable and **pass** on fixed)


## Pipeline summary (from the paper)

RUST-VGA is produced in two stages:

1) **Data Generation**
- LLM generates a vulnerable/fixed pair + a test
- We keep a pair only if:
  - both versions compile
  - test fails on vulnerable and passes on fixed

2) **Data Augmentation**
We generate additional variants using three strategies:
- **A1**: test-driven bug injection and repair
- **A2**: vulnerability-specific paraphrasing (structure changes, bug preserved)
- **A3**: contextual completion using diff-based context

### Requirements
- Rust toolchain (recommended: `rustc 1.89.0` or newer)
- Python 3.x

### Validate one sample (compile + test)
Example (replace with your actual script/paths):
```bash
cd Dataset/o3_mini/data_generation/CWE-190/pair1
cargo test


The CWE Distribution of source data used the creation of RUST-VGA. It contain 122 samples.

| CWE     | Count |
| ------- | ----: |
| CWE-022 |     3 |
| CWE-059 |     2 |
| CWE-079 |     5 |
| CWE-088 |     1 |
| CWE-113 |     1 |
| CWE-119 |     5 |
| CWE-125 |     7 |
| CWE-131 |     1 |
| CWE-134 |     1 |
| CWE-190 |     4 |
| CWE-191 |     1 |
| CWE-193 |     2 |
| CWE-20  |     4 |
| CWE-200 |     1 |
| CWE-203 |     3 |
| CWE-248 |     1 |
| CWE-252 |     2 |
| CWE-253 |     1 |
| CWE-276 |     1 |
| CWE-285 |     1 |
| CWE-287 |     2 |
| CWE-288 |     1 |
| CWE-311 |     1 |
| CWE-346 |     1 |
| CWE-347 |     3 |
| CWE-362 |     1 |
| CWE-369 |     1 |
| CWE-400 |     7 |
| CWE-415 |     2 |
| CWE-416 |     9 |
| CWE-426 |     1 |
| CWE-427 |     2 |
| CWE-444 |     2 |
| CWE-475 |     1 |
| CWE-611 |     1 |
| CWE-617 |     1 |
| CWE-662 |     1 |
| CWE-665 |     1 |
| CWE-668 |     1 |
| CWE-670 |     2 |
| CWE-674 |     2 |
| CWE-682 |     3 |
| CWE-701 |     1 |
| CWE-703 |     1 |
| CWE-754 |     2 |
| CWE-755 |     2 |
| CWE-758 |     1 |
| CWE-770 |     2 |
| CWE-787 |     6 |
| CWE-824 |     1 |
| CWE-835 |     1 |
| CWE-863 |     2 |
| CWE-908 |     2 |



## Usage

To use this dataset, clone the repository and navigate to the desired directory:

## Security and responsible use

This dataset contains intentionally vulnerable code for research purposes.
Do not deploy these samples in production environments.

## Citation:

If you use this dataset, please cite:

@misc{rustvga2026,
  title        = {RUST-VGA: Leverage LLMs to Generate a Benchmark Dataset for Rust Vulnerability Detection},
  author       = {Anonymous},
  year         = {2026},
  note         = {ACL submission}
}

