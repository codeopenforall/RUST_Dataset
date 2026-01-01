#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
A2.MakeVulnerable.py — Vulnerability-Specific Paraphrasing (env-driven, single-dir)
Generates A2_vulnerable.rs by transforming vulnerable.rs into a *different-shaped* implementation
that *reintroduces/preserves* the target CWE bug so tests FAIL, while keeping the file
compilable and public API unchanged. Copies the test file as demo_test.rs into the
destination directory for the oracles runner.

Environment (set by orchestrator):
  A2_SRC_ROOT        -> absolute/relative path to source root
  A2_DST_ROOT        -> absolute/relative path to destination root
  A2_CURRENT_SUBDIR  -> the subdirectory name being processed

Optional local dev:
  export A2_SRC_ROOT=...
  export A2_DST_ROOT=...
  export A2_CURRENT_SUBDIR=...

Requires:
  pip install -U "transformers>=4.44.2" "accelerate>=0.33.0" "torch>=2.1"
"""

import os
import re
import sys
import random as _random
from pathlib import Path
from typing import List, Optional

# ---------------- Generation config ----------------
DEFAULT_MODEL_ID = "Qwen/Qwen2.5-Coder-14B-Instruct"
GEN_MAX_NEW_TOKENS = 6000
_sys_rand = _random.SystemRandom()
TEMP_START = round(_sys_rand.uniform(0.7, 1.0), 3)
TEMP_STEP  = round(_sys_rand.uniform(0.05, 0.30), 3)
TRIES_PER_DIR = 6
SEED = _sys_rand.randint(0, 2**31 - 1)

# ---------------- System + User prompts ----------------
SYSTEM_MSG = (
    "You are an expert Rust engineer. Produce minimal, compilable code.\n"
    "Keep ALL public function names, visibility, and signatures unchanged—especially anything used by unit tests.\n"
    "No new crates; standard library only. Avoid large refactors. Ensure the file compiles.\n"
)

USER_TEMPLATE = """\
We have a Rust project with two files in the same module directory:
- vulnerable.rs       (currently fails unit test)
- demo_test.rs   (unit tests; DO NOT edit)

CWE requirement: {cwe_code}
Technique: Vulnerability-Specific Paraphrasing (produce a vulnerable implementation that preserves the bug semantics but changes the code shape, identifiers, and control structure)
Guidance: {cwe_guidance}

Unit‑Test Awareness (reference only; DO NOT edit tests):
- The tests directly/indirectly reference these public symbols (names, signatures, attributes). Preserve them exactly.
- The generated code must be compilable and must fail the unit test. 
{test_summary}

Important note about vulnerable.rs:
- You may read it to infer the *kind* of bug, but DO NOT copy code. Generate a *different* implementation that still manifests the same bug.
- MAKE SURE YOU HAVE MADE PARAPHRASING AND COUNTER CHECKED IT.

Your task:
Create **A2_vulnerable.rs** by *paraphrasing* the original implementation into a different shape that reintroduces ONE realistic defect consistent with {cwe_code} so the existing unit tests FAIL, while compiling successfully. Keep edits focused and minimal.

Rules:
- DO NOT change any public item names/signatures/visibility referenced by tests.
- Keep behavior change minimal; inject a focused defect only.
- Use only Rust standard library. No extra test code.
- Output ONLY the full source file contents.

FORMAT (STRICT):
- Reply with EXACTLY one fenced code block with the filename on the fence line:
```A2_vulnerable.rs
<entire file here>
```

--- vulnerable.rs ---
{vuln_src}

--- demo_test.rs ---
{test_src}

"""

# ---------------- Utilities ----------------
def read_text_fallback(p: Path) -> str:
    try:
        return p.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return p.read_text(encoding="latin-1")

def last_7_cwe(path: Path) -> str:
    s = str(path)
    return s[-7:] if len(s) >= 7 else s

def cwe_guidance_for(code: str) -> str:
    # No external hints table: keep a generic instruction using the code label.
    return f"{code}: Inject exactly one minimal, realistic defect consistent with this CWE, keep the file compiling, and preserve the tested public API."

TYPE_OR_STRUCT = re.compile(r"(?m)^\s*pub\s+(?:struct|enum|trait)\s+([A-Za-z_]\w*)")
USE_ITEM       = re.compile(r"(?m)^\s*pub\s+use\s+(.+);")

def summarize_test_targets(test_src: str) -> str:
    called = set(re.findall(r"(?m)(?:^|[^A-Za-z0-9_])([A-Za-z_]\w*)\s*\(", test_src))
    blacklist = {"assert", "assert_eq", "assert_ne", "println", "format", "panic", "dbg", "assert_matches", "eprintln"}
    called = {c for c in called if c not in blacklist}
    types  = set(TYPE_OR_STRUCT.findall(test_src))
    uses   = set(USE_ITEM.findall(test_src))
    lines = []
    if called:
        lines.append("- Functions called by tests: " + ", ".join(sorted(called)))
    if types:
        lines.append("- Types referenced by tests: " + ", ".join(sorted(types)))
    if uses:
        lines.append("- Public imports in tests: " + ", ".join(sorted(uses)))
    if not lines:
        return "- (No specific calls detected; preserve all public API surface exactly.)"
    return "\n".join(lines)

def load_model(model_id: str):
    try:
        import torch
        from transformers import AutoTokenizer, AutoModelForCausalLM, set_seed
    except Exception:
        print("Missing deps. Install with: pip install -U transformers torch", file=sys.stderr)
        sys.exit(1)
    set_seed(SEED)
    try:
        use_gpu = __import__("torch").cuda.is_available()
        dtype = __import__("torch").bfloat16 if use_gpu else __import__("torch").float32
    except Exception:
        dtype = None

    tok = AutoTokenizer.from_pretrained(model_id, trust_remote_code=True)
    mdl = AutoModelForCausalLM.from_pretrained(
        model_id,
        torch_dtype=dtype if dtype is not None else None,
        device_map="auto" if (hasattr(__import__('torch'), "cuda") and __import__('torch').cuda.is_available()) else None,
        trust_remote_code=True,
    )
    return tok, mdl

def apply_chat_template(tokenizer, messages) -> str:
    try:
        return tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    except Exception:
        text = ""
        for m in messages:
            text += f"<|{m.get('role','user')}|>\n{m.get('content','')}\n"
        text += "<|assistant|>\n"
        return text

def call_model(tokenizer, model, vuln_src: str, test_src: str,
               cwe_code: str, cwe_guidance: str, test_summary: str,
               temperature: float, max_new_tokens: int,
               do_sample: bool = True) -> str:
    SYSTEM = (
        SYSTEM_MSG
        + "Perform *Vulnerability-Specific Paraphrasing*: preserve the API and bug semantics, but change the implementation shape (identifiers, control flow, helpers).\n"
        + "Ensure the resulting file compiles. Inject ONE focused defect consistent with the CWE.\n"
        + "MAKE SURE YOU HAVE MADE PARAPHRASING AND COUNTER CHECKED IT"
    )
    messages = [
        {"role": "system", "content": SYSTEM},
        {"role": "user",   "content": USER_TEMPLATE.format(
            vuln_src=vuln_src,
            test_src=test_src,
            cwe_code=cwe_code,
            cwe_guidance=cwe_guidance,
            test_summary=test_summary
        )},
    ]
    prompt = apply_chat_template(tokenizer, messages)
    inputs = tokenizer(prompt, return_tensors="pt").to(model.device)
    gen_kwargs = dict(
        max_new_tokens=max_new_tokens,
        do_sample=do_sample,
        eos_token_id=tokenizer.eos_token_id,
        pad_token_id=tokenizer.eos_token_id,
        temperature=temperature if do_sample else None,
        top_p=0.9 if do_sample else None,
        repetition_penalty=1.05,
    )
    gen_kwargs = {k: v for k, v in gen_kwargs.items() if v is not None}
    outputs = model.generate(**inputs, **gen_kwargs)
    new_tokens = outputs[0, inputs["input_ids"].shape[-1]:]
    return tokenizer.decode(new_tokens, skip_special_tokens=True).strip()

FENCE_BLOCK_RE = re.compile(r"```([^\n`]*)\n(.*?)```", re.DOTALL)

def extract_vulnerable(text: str) -> str:
    m = re.search(r"```A2_vulnerable\.rs\s*(.*?)```", text, re.DOTALL | re.IGNORECASE)
    if m:
        return m.group(1).strip()
    m2 = re.search(r"```[^\n`]*\n(.*?)```", text, re.DOTALL)
    return m2.group(1).strip() if m2 else ""

# ---------------- Single-directory generation ----------------
def _pick_test_file(d: Path) -> Optional[Path]:
    for cand in ("demo_test.rs", "oracle.rs", "orcale.rs"):
        p = d / cand
        if p.exists():
            return p
    return None

def generate_vulnerable(src_dir: Path, dst_dir: Path,
                        model_id: str = DEFAULT_MODEL_ID,
                        tries: int = TRIES_PER_DIR,
                        temp_start: float = TEMP_START,
                        temp_step: float = TEMP_STEP,
                        debug: bool = False) -> bool:
    vuln_p = src_dir / "vulnerable.rs"
    test_p  = _pick_test_file(src_dir)
    if not vuln_p.exists() or test_p is None:
        print(f"[error] Missing vulnerable.rs or test file in {src_dir}", file=sys.stderr)
        return False

    vuln_p = src_dir / "vulnerable.rs"
    vuln_src = read_text_fallback(vuln_p) if vuln_p.exists() else ""
    vuln_src = read_text_fallback(vuln_p)
    test_src  = read_text_fallback(test_p)
    test_summary = summarize_test_targets(test_src)

    # Derive CWE from last 7 chars of the parent path (or blank if not in path)
    cwe_code = last_7_cwe(src_dir)
    cwe_desc = cwe_guidance_for(cwe_code) if cwe_code else "Inject one minimal, realistic defect while keeping the file compiling and API unchanged."

    tok, mdl = load_model(model_id)

    temp = temp_start
    # Attempt 1: deterministic to stabilize framing
    resp = call_model(tok, mdl, vuln_src, test_src, cwe_code, cwe_desc, test_summary,
                      temperature=temp, max_new_tokens=GEN_MAX_NEW_TOKENS, do_sample=False)
    body = extract_vulnerable(resp)
    if not body:
        if debug:
            (dst_dir / "A2_raw_attempt1.txt").write_text(resp, encoding="utf-8")
        # try sampling attempts
        for attempt in range(2, tries + 1):
            temp += temp_step
            resp = call_model(tok, mdl, vuln_src, test_src, cwe_code, cwe_desc, test_summary,
                              temperature=temp, max_new_tokens=GEN_MAX_NEW_TOKENS, do_sample=True)
            body = extract_vulnerable(resp)
            if body:
                break
            if debug:
                (dst_dir / f"A2_raw_attempt{attempt}.txt").write_text(resp, encoding="utf-8")

    if not body:
        print("[error] Model did not produce a valid fenced A2_vulnerable.rs block.", file=sys.stderr)
        return False

    # Write outputs
    (dst_dir / "A2_vulnerable.rs").write_text(body, encoding="utf-8")
    # Always copy the test file into dst as demo_test.rs
    (dst_dir / "demo_test.rs").write_text(test_src, encoding="utf-8")
    return True

# ---------------- Main (env-driven) ----------------
if __name__ == "__main__":
    SRC_ROOT = os.environ.get("A2_SRC_ROOT")
    DST_ROOT = os.environ.get("A2_DST_ROOT")
    SUB = os.environ.get("A2_CURRENT_SUBDIR")
    if not (SRC_ROOT and DST_ROOT and SUB):
        print("Missing env vars: require A2_SRC_ROOT, A2_DST_ROOT, A2_CURRENT_SUBDIR", file=sys.stderr)
        sys.exit(2)

    src_dir = Path(SRC_ROOT) / SUB
    dst_dir = Path(DST_ROOT) / SUB
    dst_dir.mkdir(parents=True, exist_ok=True)

    ok = generate_vulnerable(src_dir, dst_dir)
    sys.exit(0 if ok else 1)