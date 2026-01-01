#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
A2.MakeFixed.py — env-driven, batch mode (loads model ONCE)
Generates A2_fixed.rs by rephrasing fixed.rs so tests PASS.

Behavior
- If A2_CURRENT_SUBDIR is set: process ONLY that subdir (legacy single-dir mode).
- Otherwise: batch mode — process multiple pair* dirs under A2_SRC_ROOT, up to A2_MAX_CREATE.
- Always mirrors the test file into the destination as demo_test.rs.
- Skips pairs that already have A2_fixed.rs when A2_ONLY_MISSING=1 (default).

Environment (set by orchestrator; no CLI flags used)
  A2_SRC_ROOT        -> input root (contains CWE/pair* dirs)
  A2_DST_ROOT        -> output root (mirrors CWE/pair* dirs)
  A2_CURRENT_SUBDIR  -> (optional) name like "pair7"; when set, runs single-dir mode
  A2_MAX_CREATE      -> (optional int) max A2_fixed.rs to create in batch (0 = unlimited)
  A2_ONLY_MISSING    -> "1" (default) to skip if A2_fixed.rs already exists; "0" to overwrite
  A2_MODEL_ID        -> (optional) HF model id; default Qwen/Qwen2.5-Coder-14B-Instruct
  A2_TRIES           -> (optional int) attempts per dir (default 6)
  A2_TEMP_START      -> (optional float) initial temperature
  A2_TEMP_STEP       -> (optional float) temperature step per retry
  A2_SEED            -> (optional int) RNG seed

Requires:
  pip install -U "transformers>=4.44.0" "torch>=2.1"
"""

import os, re, sys, shutil, random as _random, textwrap
from pathlib import Path
from typing import List

# -------- Config (env-overridable) --------
DEFAULT_MODEL_ID = os.environ.get("A2_MODEL_ID", "Qwen/Qwen2.5-Coder-14B-Instruct")
GEN_MAX_NEW_TOKENS = 6000
TRIES_PER_DIR = int(os.environ.get("A2_TRIES", "6"))
TEMP_START = float(os.environ.get("A2_TEMP_START", str(round(_random.uniform(0.1, 1.0), 3))))
TEMP_STEP  = float(os.environ.get("A2_TEMP_STEP",  str(round(_random.uniform(0.05, 0.30), 3))))
SEED       = int(os.environ.get("A2_SEED", str(_random.randint(0, 2**31 - 1))))

# -------- Prompt scaffolding --------
SYSTEM_MSG = (
    "You are an expert Rust engineer. Produce minimal, compilable edits.\n"
    "Keep all public function names and signatures unchanged—especially anything used by unit tests.\n"
    "No new crates; standard library only. Prefer small, localized changes.\n"
)

USER_TEMPLATE = """\
We have two files in a Rust mini-project:
- fixed.rs  (currently passes the unit tests)
- demo_test.rs   (unit tests; DO NOT edit)

Goal:
Create **A2_fixed.rs** by minimally rephrasing the code so that it can still passes the test.

Rules:
- Do NOT change any public names/signatures/types referenced by the unit tests.
- Keep the code compiling;
- Avoid large refactors; edit minimally.
- Use ONLY the standard library.
- Return EXACTLY one fenced code block with the filename on the fence line:
- The generated code must be compilable and must pass the test.

```A2_fixed.rs
<entire file content here>
```

--- fixed.rs ---
{fixed_src}

--- demo_test.rs ---
{test_src}
"""

# -------- Utilities --------
def read_text_fallback(p: Path) -> str:
    try:
        return p.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return p.read_text(encoding="latin-1")

def extract_fixed(text: str) -> str:
    m = re.search(r"```A2_fixed\.rs\s*(.*?)```", text, re.DOTALL | re.IGNORECASE)
    if m:
        return m.group(1).strip()
    m2 = re.search(r"```[^\n`]*\n(.*?)```", text, re.DOTALL)
    return m2.group(1).strip() if m2 else ""

def apply_chat_template(tokenizer, messages) -> str:
    try:
        return tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    except Exception:
        text = ""
        for m in messages:
            role = m.get("role", "user")
            content = m.get("content", "")
            text += f"<|{role}|>\n{content}\n"
        text += "<|assistant|>\n"
        return text

# -------- Model (load once) --------
def load_model_once(model_id: str):
    try:
        import torch
        from transformers import AutoTokenizer, AutoModelForCausalLM, set_seed
    except Exception as e:
        print("Missing deps. Install with: pip install -U transformers torch", file=sys.stderr)
        print(str(e), file=sys.stderr)
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
    return tok, mdl

def call_model(tokenizer, model, fixed_src: str, test_src: str,
               temperature: float, max_new_tokens: int, do_sample: bool) -> str:
    messages = [
        {"role": "system", "content": SYSTEM_MSG},
        {"role": "user",   "content": USER_TEMPLATE.format(fixed_src=fixed_src, test_src=test_src)},
    ]
    prompt = apply_chat_template(tokenizer, messages)
    inputs = tokenizer(prompt, return_tensors="pt").to(model.device)
    outputs = model.generate(
        **inputs,
        max_new_tokens=max_new_tokens,
        do_sample=do_sample,
        temperature=(temperature if do_sample else None),
        top_p=(0.9 if do_sample else None),
        eos_token_id=tokenizer.eos_token_id,
        pad_token_id=tokenizer.eos_token_id,
        repetition_penalty=1.05,
    )
    new_tokens = outputs[0, inputs["input_ids"].shape[-1]:]
    return tokenizer.decode(new_tokens, skip_special_tokens=True).strip()

# -------- Discovery --------
PAIR_RE = re.compile(r"^pair\d+$")

def find_candidate_dirs(root: Path) -> List[Path]:
    cands: List[Path] = []
    if not root.exists():
        return cands
    for d in root.iterdir():
        if not (d.is_dir() and PAIR_RE.match(d.name)):
            continue
        names = {p.name for p in d.iterdir() if p.is_file()}
        if "fixed.rs" in names and any(n in names for n in ("demo_test.rs", "oracle.rs", "orcale.rs")):
            cands.append(d)
    cands.sort()
    return cands

# -------- Per-dir processing (reusing loaded model) --------
def process_dir_with_model(src_dir: Path, in_root: Path, out_root: Path, tok, mdl,
                           tries: int, t0: float, tstep: float, only_missing: bool) -> bool:
    fixed_p = src_dir / "fixed.rs"
    test_p = None
    for nm in ("demo_test.rs", "oracle.rs", "orcale.rs"):
        cand = src_dir / nm
        if cand.exists():
            test_p = cand
            break
    if not (fixed_p.exists() and test_p and test_p.exists()):
        return False

    rel = src_dir.relative_to(in_root)
    out_dir = out_root / rel
    out_dir.mkdir(parents=True, exist_ok=True)

    if only_missing and (out_dir / "A2_fixed.rs").exists():
        return False

    fixed_src = read_text_fallback(fixed_p)
    test_src = read_text_fallback(test_p)

    temp = t0
    # Attempt 1: no sampling to stabilize formatting
    resp = call_model(tok, mdl, fixed_src, test_src, temperature=temp, max_new_tokens=GEN_MAX_NEW_TOKENS, do_sample=False)
    body = extract_fixed(resp)
    if body:
        (out_dir / "A2_fixed.rs").write_text(body, encoding="utf-8")
        (out_dir / "demo_test.rs").write_text(test_src, encoding="utf-8")
        return True

    # Attempts with sampling and temperature ramp
    for _ in range(2, tries + 1):
        temp += tstep
        resp = call_model(tok, mdl, fixed_src, test_src, temperature=temp, max_new_tokens=GEN_MAX_NEW_TOKENS, do_sample=True)
        body = extract_fixed(resp)
        if body:
            (out_dir / "A2_fixed.rs").write_text(body, encoding="utf-8")
            (out_dir / "demo_test.rs").write_text(test_src, encoding="utf-8")
            return True

    return False

# -------- Main --------
def main():
    SRC_ROOT = os.environ.get("A2_SRC_ROOT")
    DST_ROOT = os.environ.get("A2_DST_ROOT")
    if not (SRC_ROOT and DST_ROOT):
        print("Missing env vars: A2_SRC_ROOT and A2_DST_ROOT", file=sys.stderr)
        sys.exit(2)

    in_root = Path(SRC_ROOT).expanduser().resolve()
    out_root = Path(DST_ROOT).expanduser().resolve()
    out_root.mkdir(parents=True, exist_ok=True)

    only_missing = os.environ.get("A2_ONLY_MISSING", "1") == "1"
    max_create = int(os.environ.get("A2_MAX_CREATE", "0"))  # 0 => unlimited
    current = os.environ.get("A2_CURRENT_SUBDIR")

    # Load model ONCE
    tok, mdl = load_model_once(DEFAULT_MODEL_ID)

    created = 0
    if current:
        src_dir = in_root / current
        ok = process_dir_with_model(src_dir, in_root, out_root, tok, mdl, TRIES_PER_DIR, TEMP_START, TEMP_STEP, only_missing)
        sys.exit(0 if ok else 1)

    # Batch mode
    for d in find_candidate_dirs(in_root):
        if max_create and created >= max_create:
            break
        # ensure test is mirrored to destination for downstream tools
        dst_d = out_root / d.name
        dst_d.mkdir(parents=True, exist_ok=True)
        if not (dst_d / "demo_test.rs").exists():
            for nm in ("demo_test.rs", "oracle.rs", "orcale.rs"):
                tp = d / nm
                if tp.exists():
                    shutil.copy2(tp, dst_d / "demo_test.rs")
                    break

        # skip if exists and only_missing
        if (dst_d / "A2_fixed.rs").exists() and only_missing:
            continue

        ok = process_dir_with_model(d, in_root, out_root, tok, mdl, TRIES_PER_DIR, TEMP_START, TEMP_STEP, only_missing)
        if ok:
            created += 1
            print(f"[created] {d.name}")

    print(f"[summary] created {created} A2_fixed.rs files")
    sys.exit(0)

if __name__ == "__main__":
    main()