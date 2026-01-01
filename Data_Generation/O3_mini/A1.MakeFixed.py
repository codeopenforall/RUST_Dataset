#!/usr/bin/env python3
"""
A1.MakeFixed.py
Generate ONLY A1_fixed.rs (repair vulnerable.rs so tests PASS).

- Reads per-project pair: vulnerable.rs and test file (demo_test.rs or oracle.rs/orcale.rs). fixed.rs is optional here.
- Writes to mirrored OUTPUT dir:
    * A1_fixed.rs   (generated)
    * demo_test.rs  (copied from input test file, regardless of original name)
- No logs, no verification, no extra artifacts.

Dependencies:
  pip install -U "transformers>=4.44.0" "torch>=2.1"  # For CUDA 12.x: install the torch build matching your system
  # Model: Qwen/Qwen2.5-Coder-14B-Instruct  (FP16/bfloat16 recommended with GPU)
"""

import os
import re
import sys
import argparse
from pathlib import Path
from typing import List

# ---------------------- DEFAULT PATHS & SETTINGS ----------------------

DEFAULT_INPUT  = "/home/ikqnm/PycharmProjects/PythonProject/DataGenerated_pairs/CWE-665"
DEFAULT_OUTPUT = "/home/ikqnm/PycharmProjects/PythonProject/deepseek/DataAugmented_pairs_new/CWE-665"

# Use Qwen 2.5 Coder 14B Instruct by default
DEFAULT_MODEL_ID = "Qwen/Qwen2.5-Coder-14B-Instruct"

# Generation setup
GEN_MAX_NEW_TOKENS = 6000
# Randomize generation hyperparameters each run:
# - TEMP_START: initial sampling temperature (0.3 .. 1.0)
# - TEMP_STEP: incremental temperature step per retry (0.05 .. 0.30)
# - SEED: random integer seed for deterministic model APIs (if used)
import random as _random
_sys_rand = _random.SystemRandom()
TEMP_START = round(_sys_rand.uniform(0.1, 1.0), 3)
TEMP_STEP  = round(_sys_rand.uniform(0.05, 0.30), 3)
TRIES_PER_DIR      = 6
SEED               = _sys_rand.randint(0, 2**31 - 1)

# ----------------------------- PROMPTS ------------------------------

SYSTEM_MSG = (
    "You are an expert Rust engineer. Produce minimal, compilable edits.\n"
    "Keep all public function names and signatures unchanged—especially anything used by the unit tests.\n"
    "No new crates; standard library only. Prefer small, localized changes.\n"
)

USER_TEMPLATE = """\
We have two files in a Rust mini-project:
- vulnerable.rs  (currently fails unit tests)
- demo_test.rs   (unit tests; DO NOT edit)

Goal:
Create **A1_fixed.rs** by minimally repairing the code so that the tests PASS.

Rules:
- Do NOT change any public names/signatures/types referenced by the unit tests.
- Keep the code compiling; fix the actual root cause of the test failure (e.g., off-by-one, logical inversion, proper error handling, synchronization, unsafe removal, etc.).
- Avoid large refactors; edit minimally.
- Use ONLY the standard library.
- Return EXACTLY one fenced code block with the filename on the fence line:
- The generated code must pass the Test.

```A1_fixed.rs
<entire file content here>
```

--- vulnerable.rs ---
{vuln_src}

--- demo_test.rs ---
{test_src}
"""

# ---------------------------- UTILITIES -----------------------------

def read_text_fallback(p: Path) -> str:
    try:
        return p.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return p.read_text(encoding="latin-1")

def load_model(model_id: str):
    try:
        import torch
        from transformers import AutoTokenizer, AutoModelForCausalLM, set_seed
    except Exception:
        print("Missing deps. Install with: pip install -U transformers torch", file=sys.stderr)
        sys.exit(1)

    set_seed(SEED)
    # Qwen supports bf16/float16 on GPU. Fall back to float32 on CPU.
    if torch.cuda.is_available():
        dtype = torch.bfloat16 if torch.cuda.is_bf16_supported() else torch.float16
        device_map = "auto"
    else:
        dtype = torch.float32
        device_map = None

    tok = AutoTokenizer.from_pretrained(model_id, trust_remote_code=True)
    mdl = AutoModelForCausalLM.from_pretrained(
        model_id,
        torch_dtype=dtype,
        device_map=device_map,
        trust_remote_code=True,
    )
    return tok, mdl

def apply_chat_template(tokenizer, messages) -> str:
    # Qwen uses HF chat templates; fall back to a simple format if unavailable.
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

def call_model(tokenizer, model, vuln_src: str, test_src: str,
               temperature: float, max_new_tokens: int) -> str:
    messages = [
        {"role": "system", "content": SYSTEM_MSG},
        {"role": "user",   "content": USER_TEMPLATE.format(vuln_src=vuln_src, test_src=test_src)},
    ]
    prompt = apply_chat_template(tokenizer, messages)
    inputs = tokenizer(prompt, return_tensors="pt").to(model.device)
    # Qwen generally supports typical sampling args.
    outputs = model.generate(
        **inputs,
        max_new_tokens=max_new_tokens,
        do_sample=True,
        temperature=temperature,
        top_p=0.9,
        eos_token_id=tokenizer.eos_token_id,
        pad_token_id=tokenizer.eos_token_id,
    )
    new_tokens = outputs[0, inputs["input_ids"].shape[-1]:]
    return tokenizer.decode(new_tokens, skip_special_tokens=True).strip()

def extract_fixed(text: str) -> str:
    m = re.search(r"```A1_fixed\.rs\s*(.*?)```", text, re.DOTALL | re.IGNORECASE)
    if m:
        return m.group(1).strip()
    m2 = re.search(r"```[^\n`]*\n(.*?)```", text, re.DOTALL)
    return m2.group(1).strip() if m2 else ""

# ----------------------------- DISCOVERY ----------------------------

def find_candidate_dirs(root: Path) -> List[Path]:
    cands = []
    for dirpath, _dirnames, filenames in os.walk(root):
        if "vulnerable.rs" in filenames and any(n in filenames for n in ("demo_test.rs", "oracle.rs", "orcale.rs")):
            cands.append(Path(dirpath))
    cands.sort()
    return cands

# --------------------------- PROCESS ONE DIR ------------------------

def process_dir(d: Path, in_root: Path, out_root: Path, tok, mdl,
                tries: int, t0: float, tstep: float, only_missing: bool) -> dict:
    vuln_p = d / "vulnerable.rs"
    if (d / "demo_test.rs").exists():
        test_p = d / "demo_test.rs"
    elif (d / "oracle.rs").exists():
        test_p = d / "oracle.rs"
    elif (d / "orcale.rs").exists():
        test_p = d / "orcale.rs"
    else:
        return {"dir": str(d), "status": "skip", "reason": "no test file"}

    rel = d.relative_to(in_root)
    out_dir = out_root / rel
    out_dir.mkdir(parents=True, exist_ok=True)

    if only_missing and (out_dir / "A1_fixed.rs").exists():
        return {"dir": str(d), "status": "skip", "reason": "existing A1_fixed.rs"}

    vuln_src = read_text_fallback(vuln_p)
    test_src = read_text_fallback(test_p)

    temp = t0
    for _ in range(tries):
        resp = call_model(tok, mdl, vuln_src, test_src, temperature=temp, max_new_tokens=GEN_MAX_NEW_TOKENS)
        body = extract_fixed(resp)
        if body:
            (out_dir / "A1_fixed.rs").write_text(body, encoding="utf-8")
            # Always mirror the test under a stable name
            (out_dir / "demo_test.rs").write_text(test_src, encoding="utf-8")
            return {"dir": str(d), "status": "ok"}
        temp += tstep
    return {"dir": str(d), "status": "failed"}

# -------------------------------- MAIN ------------------------------

def main():
    ap = argparse.ArgumentParser(description="Generate A1_fixed.rs from vulnerable.rs + tests using Qwen2.5-Coder-14B-Instruct.")
    ap.add_argument("--input",  default=DEFAULT_INPUT,  help="Input root or CWE directory")
    ap.add_argument("--output", default=DEFAULT_OUTPUT, help="Output root (mirrors input)")
    ap.add_argument("--model",  default=DEFAULT_MODEL_ID, help="Hugging Face model id")
    ap.add_argument("--tries",  type=int, default=TRIES_PER_DIR, help="Max attempts per directory")
    ap.add_argument("--temp",   type=float, default=TEMP_START, help="Initial temperature")
    ap.add_argument("--temp-step", type=float, default=TEMP_STEP, help="Temperature step per retry")
    ap.add_argument("--only-missing", action="store_true", help="Skip dirs that already have A1_fixed.rs")
    args = ap.parse_args()

    in_root  = Path(args.input).expanduser().resolve()
    out_root = Path(args.output).expanduser().resolve()
    out_root.mkdir(parents=True, exist_ok=True)

    print(f"[info] Input : {in_root}")
    print(f"[info] Output: {out_root}")
    print(f"[info] Model : {args.model}")

    tok, mdl = load_model(args.model)
    dirs = find_candidate_dirs(in_root)
    if not dirs:
        print("[warn] No candidate directories found."); sys.exit(0)

    total = ok = failed = skipped = 0
    for d in dirs:
        total += 1
        rel = d.relative_to(in_root)
        print(f"[{total}] A1_fixed for: {rel}")
        res = process_dir(d, in_root, out_root, tok, mdl, args.tries, args.temp, args.temp_step, args.only_missing)
        st = res.get("status")
        if st == "ok":
            ok += 1
            print("    OK")
        elif st == "failed":
            failed += 1
            print("    FAILED")
        else:
            skipped += 1
            print(f"    SKIPPED ({res.get('reason','')})")

    print("\n[summary]")
    print(f"  processed : {total}")
    print(f"  ok        : {ok}")
    print(f"  failed    : {failed}")
    print(f"  skipped   : {skipped}")
    print(f"  out root  : {out_root}")

if __name__ == "__main__":
    main()