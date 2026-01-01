#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
A3.MakeFixed.py — recursive, source-driven (loads model ONCE)

Generates A3_fixed.rs for each source subdir that contains:
  - fixed_diff.rs (required, primary context)
  - demo_test.rs  (required, tests; mirrored to dest)
Optionally uses:
  - vulnerable_diff.rs (secondary context)
Also mirrors the diff files into DEST for traceability.

Behavior
- If A3_CURRENT_SUBDIR is set: process ONLY that subdir (relative to A3_SRC_ROOT).
- Otherwise: recursive batch — walk A3_SRC_ROOT and process every directory
  containing fixed_diff.rs + demo_test.rs.
- Writes into A3_DST_ROOT mirroring the relative path.
- Skips dirs with existing A3_fixed.rs when A3_ONLY_MISSING=1 (default).

Environment (no CLI flags)
  A3_SRC_ROOT        -> input root (contains CWE/pair* dirs)
  A3_DST_ROOT        -> output root (mirrors structure)
  A3_CURRENT_SUBDIR  -> (optional) relative path like "CWE-X" or just "pair7"
  A3_MAX_CREATE      -> (optional int) max A3_fixed.rs to create in batch (0 = unlimited)
  A3_ONLY_MISSING    -> "1" (default) skip if A3_fixed.rs exists; "0" overwrite
  A3_MODEL_ID        -> (optional) HF model id; default Qwen/Qwen2.5-Coder-14B-Instruct
  A3_TRIES           -> (optional int) attempts per dir (default 6)
  A3_TEMP_START      -> (optional float) initial temperature
  A3_TEMP_STEP       -> (optional float) temperature step per retry
  A3_SEED            -> (optional int) RNG seed
  A3_OVERWRITE_TEST  -> "0"/"1" overwrite dest demo_test.rs (default "1")

Requires:
  pip install -U "transformers>=4.44.0" "torch>=2.1"
"""

import os, re, sys, shutil, traceback, random as _random
from pathlib import Path
from typing import List, Optional
from datetime import datetime

# ---------- Config (env-overridable) ----------
DEFAULT_MODEL_ID = os.environ.get("A3_MODEL_ID", "Qwen/Qwen2.5-Coder-14B-Instruct")
GEN_MAX_NEW_TOKENS = 6000
TRIES_PER_DIR = int(os.environ.get("A3_TRIES", "6"))
TEMP_START = float(os.environ.get("A3_TEMP_START", str(round(_random.uniform(0.10, 0.90), 3))))
TEMP_STEP  = float(os.environ.get("A3_TEMP_STEP",  str(round(_random.uniform(0.05, 0.30), 3))))
SEED       = int(os.environ.get("A3_SEED", str(_random.randint(0, 2**31 - 1))))
OVERWRITE_TEST = os.environ.get("A3_OVERWRITE_TEST", "1") == "1"

cwe_name = "CWE 908"
# ---------- Logging ----------
def log(msg: str, level: str = "INFO"):
    ts = datetime.now().strftime("%H:%M:%S")
    tag = f"[{level}]".ljust(7)
    print(f"{ts} {tag} {msg}")

# ---------- Prompt scaffolding ----------
SYSTEM_MSG = (
    "You are a senior Rust engineer. Produce minimal, compilable code that passes the provided unit tests.\n"
    "Keep all public function names and signatures unchanged—especially anything used by unit tests.\n"
    "No new crates; standard library only. Prefer small, localized changes. Idiomatic, clear Rust.\n"
    "The code should be completed based on the following CWE\n"
    "CWE-908"
)

USER_TEMPLATE = """\
You are completing a Rust mini-project using DIFF CONTEXT and TESTS.

Files available:
- fixed_diff.rs        (DIFF lines that exist in 'fixed.rs' but NOT in 'vulnerable.rs') — PRIMARY CONTEXT
- vulnerable_diff.rs   (optional: lines from 'vulnerable.rs' not in 'fixed.rs') — SECONDARY CONTEXT
- demo_test.rs         (UNIT TESTS; DO NOT MODIFY)

{maybe_cwe}
Goal:
Create **A3_fixed.rs** — a complete, compilable Rust source file that passes the test in demo_test.rs, make sure you generate the same method/methods used demo_test.rs.
Use the information in fixed_diff.rs to reconstruct or rephrase the intended fixed behavior.
If needed, infer minimal missing parts so that the program compiles and tests pass.

Rules:
- KEEP all public names/signatures/types that tests rely on.
- Keep changes minimal and localized.
- Use ONLY the standard library; no external crates.
- Return EXACTLY one fenced code block with the filename on the fence line.
- The code block must contain the FULL content of A3_fixed.rs.

```A3_fixed.rs
<entire file content here>
```

--- fixed_diff.rs (PRIMARY) ---
{fixed_diff_src}

--- vulnerable_diff.rs (optional) ---
{vuln_diff_src}

--- demo_test.rs (tests; DO NOT modify) ---
{test_src}
"""

def build_user_prompt(fixed_diff: str, test_src: str, vuln_diff: Optional[str], cwe_name: Optional[str]) -> str:
    maybe_cwe = f"Context: This example belongs to **{cwe_name}**.\n" if cwe_name else ""
    return USER_TEMPLATE.format(
        fixed_diff_src = fixed_diff.strip() if fixed_diff else "(empty)",
        vuln_diff_src  = (vuln_diff.strip() if vuln_diff else "(absent)"),
        test_src       = test_src.strip() if test_src else "(missing tests!)",
        maybe_cwe      = maybe_cwe
    )

# ---------- Utilities ----------
def read_text(p: Path) -> str:
    try:
        return p.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return p.read_text(encoding="latin-1")
    except Exception:
        return ""

def extract_generated(text: str) -> str:
    m = re.search(r"```A3_fixed\.rs\s*(.*?)```", text, re.DOTALL | re.IGNORECASE)
    if m:
        return m.group(1).strip()
    m2 = re.search(r"```[^\n`]*\n(.*?)```", text, re.DOTALL)
    return m2.group(1).strip() if m2 else ""

def apply_chat_template(tokenizer, messages) -> str:
    try:
        return tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    except Exception:
        txt = ""
        for m in messages:
            txt += f"<|{m.get('role','user')}|>\n{m.get('content','')}\n"
        txt += "<|assistant|>\n"
        return txt

# ---------- Model (load once) ----------
def load_model_once(model_id: str):
    try:
        import torch
        from transformers import AutoTokenizer, AutoModelForCausalLM, set_seed
    except Exception as e:
        log("Missing deps. Install with: pip install -U transformers torch", "ERROR")
        log(str(e), "ERROR")
        sys.exit(1)
    set_seed(SEED)
    try:
        use_gpu = __import__("torch").cuda.is_available()
    except Exception:
        use_gpu = False
    dtype = __import__("torch").bfloat16 if use_gpu else __import__("torch").float32
    tok = AutoTokenizer.from_pretrained(model_id, trust_remote_code=True)
    mdl = AutoModelForCausalLM.from_pretrained(
        model_id,
        torch_dtype=dtype,
        device_map="auto" if use_gpu else None,
        trust_remote_code=True,
    )
    log(f"Model loaded: {model_id} (GPU={use_gpu})")
    return tok, mdl

def call_model(tokenizer, model, fixed_diff: str, vuln_diff: Optional[str], test_src: str,
               cwe_name: Optional[str], temperature: float, do_sample: bool) -> str:
    messages = [
        {"role": "system", "content": SYSTEM_MSG},
        {"role": "user",   "content": build_user_prompt(fixed_diff, test_src, vuln_diff, cwe_name)},
    ]
    prompt = apply_chat_template(tokenizer, messages)
    inputs = tokenizer(prompt, return_tensors="pt").to(model.device)
    outputs = model.generate(
        **inputs,
        max_new_tokens=GEN_MAX_NEW_TOKENS,
        do_sample=do_sample,
        temperature=(temperature if do_sample else None),
        top_p=(0.9 if do_sample else None),
        eos_token_id=tokenizer.eos_token_id,
        pad_token_id=tokenizer.eos_token_id,
        repetition_penalty=1.05,
    )
    new_tokens = outputs[0, inputs["input_ids"].shape[-1]:]
    return tokenizer.decode(new_tokens, skip_special_tokens=True).strip()

# ---------- Discovery ----------
def dir_has_required(d: Path) -> bool:
    names = {p.name for p in d.iterdir() if p.is_file()}
    return ("fixed_diff.rs" in names) and ("demo_test.rs" in names)

def find_candidate_dirs_recursive(root: Path) -> List[Path]:
    cands: List[Path] = []
    if not root.exists():
        return cands
    for d in root.rglob("*"):
        if d.is_dir() and dir_has_required(d):
            cands.append(d)
    cands.sort()
    return cands

def guess_cwe_from_path(p: Path) -> Optional[str]:
    for part in p.parts:
        if part.upper().startswith("CWE-"):
            return part
    return None

# ---------- Per-dir processing ----------
def process_dir(src_dir: Path, in_root: Path, out_root: Path, tok, mdl,
                tries: int, t0: float, tstep: float, only_missing: bool) -> bool:
    fixed_diff_p = src_dir / "fixed_diff.rs"
    test_p       = src_dir / "demo_test.rs"
    vuln_diff_p  = src_dir / "vulnerable_diff.rs"  # optional

    rel = src_dir.relative_to(in_root)
    out_dir = out_root / rel
    out_dir.mkdir(parents=True, exist_ok=True)

    out_rs = out_dir / "A3_fixed.rs"

    log(f"— Processing: {rel}", "STEP")
    if not (fixed_diff_p.exists() and test_p.exists()):
        log("Skip: required files missing", "WARN")
        return False

    if out_rs.exists() and only_missing:
        log(f"Skip (exists & ONLY_MISSING=1): {out_rs}", "WARN")
        # still mirror test for consistency
        dst_test = out_dir / "demo_test.rs"
        if OVERWRITE_TEST or not dst_test.exists():
            shutil.copy2(test_p, dst_test)
            log("Mirrored test file (existing A3_fixed.rs).")
        # mirror diffs too for traceability
        try:
            shutil.copy2(fixed_diff_p, out_dir / "fixed_diff.rs")
            if vuln_diff_p.exists():
                shutil.copy2(vuln_diff_p, out_dir / "vulnerable_diff.rs")
        except Exception as e:
            log(f"Copy diff files failed: {e}", "ERROR")
        return False

    fixed_diff = read_text(fixed_diff_p)
    test_src   = read_text(test_p)
    vuln_diff  = read_text(vuln_diff_p) if vuln_diff_p.exists() else None
    cwe_name   = guess_cwe_from_path(src_dir)

    # Mirror test & diffs to DEST (for auditing)
    dst_test = out_dir / "demo_test.rs"
    if OVERWRITE_TEST or not dst_test.exists():
        shutil.copy2(test_p, dst_test)
        log(f"Mirrored test → {dst_test}")
    try:
        shutil.copy2(fixed_diff_p, out_dir / "fixed_diff.rs")
        log("Mirrored fixed_diff.rs")
        if vuln_diff_p.exists():
            shutil.copy2(vuln_diff_p, out_dir / "vulnerable_diff.rs")
            log("Mirrored vulnerable_diff.rs")
    except Exception as e:
        log(f"Diff mirroring failed: {e}", "ERROR")

    # Try generation
    temp = t0
    log(f"Generate A3_fixed.rs (tries={tries}, t0={t0}, step={tstep})")
    try:
        resp = call_model(tok, mdl, fixed_diff, vuln_diff, test_src, cwe_name,
                          temperature=temp, do_sample=False)
        body = extract_generated(resp)
        if body:
            out_rs.write_text(body, encoding="utf-8")
            log(f"[CREATED] {out_rs} (deterministic)")
            return True
    except Exception as e:
        log(f"First attempt failed: {e}", "ERROR")
        log(traceback.format_exc(), "ERROR")

    for attempt in range(2, tries + 1):
        temp += tstep
        try:
            resp = call_model(tok, mdl, fixed_diff, vuln_diff, test_src, cwe_name,
                              temperature=temp, do_sample=True)
            body = extract_generated(resp)
            if body:
                out_rs.write_text(body, encoding="utf-8")
                log(f"[CREATED] {out_rs} (attempt {attempt}, T={temp:.3f})")
                return True
        except Exception as e:
            log(f"Attempt {attempt} failed: {e}", "ERROR")
            log(traceback.format_exc(), "ERROR")

    log("FAILED to create A3_fixed.rs", "ERROR")
    return False

# ---------- Main ----------
def main():
    SRC_ROOT = os.environ.get("A3_SRC_ROOT")
    DST_ROOT = os.environ.get("A3_DST_ROOT")
    if not (SRC_ROOT and DST_ROOT):
        log("Missing env vars: A3_SRC_ROOT and A3_DST_ROOT", "ERROR")
        sys.exit(2)

    in_root = Path(SRC_ROOT).expanduser().resolve()
    out_root = Path(DST_ROOT).expanduser().resolve()
    out_root.mkdir(parents=True, exist_ok=True)

    only_missing = os.environ.get("A3_ONLY_MISSING", "1") == "1"
    max_create   = int(os.environ.get("A3_MAX_CREATE", "0"))  # 0 => unlimited
    current_rel  = os.environ.get("A3_CURRENT_SUBDIR")

    # load model once
    tok, mdl = load_model_once(DEFAULT_MODEL_ID)

    created = 0

    # Single-dir mode
    if current_rel:
        cand = (in_root / current_rel)
        src_dir = cand if cand.exists() else next((p for p in in_root.rglob(current_rel) if p.is_dir()), cand)
        ok = process_dir(src_dir, in_root, out_root, tok, mdl, TRIES_PER_DIR, TEMP_START, TEMP_STEP, only_missing)
        sys.exit(0 if ok else 1)

    # Batch mode
    for d in find_candidate_dirs_recursive(in_root):
        if max_create and created >= max_create:
            break
        ok = process_dir(d, in_root, out_root, tok, mdl, TRIES_PER_DIR, TEMP_START, TEMP_STEP, only_missing)
        if ok:
            created += 1
            log(f"[summary] created {created} so far", "INFO")

    log(f"[summary] created {created} A3_fixed.rs files total", "DONE")
    sys.exit(0)

if __name__ == "__main__":
    main()