# RUST Dataset
This repository contains the dataset used in the paper **"RUST-VGA: Leverage LLMs to Generate a Benchmark Dataset for Rust**"
annotated with CVE and CWE labels for systematic code generation and augmentation, and for the evaluation of vulnerability detection methods.

## Overview
The dataset includes Rust code samples, both vulnerable and non-vulnerable.

## Structure

The dataset is organized into the following directories:

- `Positive/`: Contains Rust code samples with known vulnerabilities.
- `Negative/`: Contains Rust code samples without known vulnerabilities.

Each file is named according to the CVE identifier and the CWE type.

The CWE Distribution of source data used the creation of RUST-VGA. It contain 136 samples.

| CWE     | Count |
| ------- | ----: |
| CWE-22  |     3 |
| CWE-59  |     2 |
| CWE-79  |     5 |
| CWE-88  |     1 |
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
