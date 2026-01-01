#!/usr/bin/env python3
"""
A1.DataAugmentation.py
DeepSeek-based Rust data augmentation (A1 policy; generation-only).

- Reads triplets: fixed.rs, vulnerable.rs, and a test file (demo_test.rs or oracle.rs/orcale.rs).
- Generates:
    * A1_vulnerable.rs  (introduce a small realistic bug into fixed.rs; should FAIL tests)
    * A1_fixed.rs       (fix vulnerable.rs; should PASS tests)
    * demo_test.rs      (copied from the input test file, regardless of original name)
- Writes these files into the mirrored OUTPUT tree.
- No logs, no extra attempt dumps.

Dependencies:
  pip install -U transformers torch
"""

import os
import re
import sys
import argparse
from pathlib import Path
from typing import Tuple, Optional, List

# ---------------------- DEFAULT PATHS & SETTINGS ----------------------

DEFAULT_INPUT  = "/home/ikqnm/PycharmProjects/PythonProject/O3_mini/DataGenerated_pairs_new/CWE-020"
DEFAULT_OUTPUT = "/home/ikqnm/PycharmProjects/PythonProject/deepseek/DataAugmented_pairs_new/CWE-020"

DEFAULT_MODEL_ID = "deepseek-ai/deepseek-coder-6.7b-instruct"  # override with --model

# Generation settings
GEN_MAX_NEW_TOKENS = 3000
TEMP_START         = 0.4
TEMP_STEP          = 0.2
TRIES_PER_DIR      = 5       # retry parse with higher temperature

# Reproducibility
SEED = 42

# ----------------------------- PROMPTS ------------------------------

SYSTEM_MSG = (
    "You are an expert Rust engineer. Produce minimal, compilable edits.\n"
    "Keep all public function names and signatures unchanged—especially anything used by the unit tests.\n"
    "No new crates; standard library only. Prefer small, localized changes.\n"
)

USER_TEMPLATE = """\
We have three files in a Rust mini-project:
- fixed.rs       (currently passes unit tests)
- vulnerable.rs  (currently fails unit tests)
- demo_test.rs   (unit tests; DO NOT edit)

Goal A1:
1) Create A1_vulnerable.rs by introducing ONE small realistic bug into fixed.rs so that the tests FAIL.
2) Create A1_fixed.rs by fixing the bug in vulnerable.rs so that the tests PASS.

Rules:
- Do NOT change public names/signatures/types referenced by the unit tests.
- Avoid large refactors; edit minimally.
- Use ONLY the standard library.
- Return EXACTLY two fenced code blocks, each with the filename on the fence line:

```A1_vulnerable.rs
<entire file content here>
```

```A1_fixed.rs
<entire file content here>
```

--- fixed.rs ---
{fixed_src}

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
    """
    Load tokenizer & model. Exits with a helpful message if deps are missing.
    """
    try:
        import torch
        from transformers import AutoTokenizer, AutoModelForCausalLM, set_seed
    except Exception:
        print("Missing deps. Please install:", file=sys.stderr)
        print("  pip install -U transformers torch", file=sys.stderr)
        sys.exit(1)

    set_seed(SEED)
    dtype = torch.bfloat16 if torch.cuda.is_available() else torch.float32
    tok = AutoTokenizer.from_pretrained(model_id, trust_remote_code=True)
    mdl = AutoModelForCausalLM.from_pretrained(
        model_id,
        torch_dtype=dtype,
        device_map="auto" if torch.cuda.is_available() else None,
        trust_remote_code=True,
    )
    return tok, mdl

def apply_chat_template(tokenizer, messages) -> str:
    """
    Prefer the model's chat template; otherwise fall back to a simple format.
    """
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

def call_model(tokenizer, model, fixed_src: str, vuln_src: str, test_src: str,
               temperature: float, max_new_tokens: int) -> str:
    """
    Build a chat prompt via apply_chat_template, generate, and return ONLY the
    assistant's response (excluding the prompt).
    """
    messages = [
        {"role": "system", "content": SYSTEM_MSG},
        {"role": "user",   "content": USER_TEMPLATE.format(
            fixed_src=fixed_src, vuln_src=vuln_src, test_src=test_src)}
    ]

    prompt = apply_chat_template(tokenizer, messages)
    inputs = tokenizer(prompt, return_tensors="pt").to(model.device)

    outputs = model.generate(
        **inputs,
        max_new_tokens=max_new_tokens,
        do_sample=True,
        temperature=temperature,
        top_p=0.9,
        eos_token_id=tokenizer.eos_token_id,
        pad_token_id=tokenizer.eos_token_id,
    )

    # Decode ONLY the assistant's new tokens (strip the prompt part)
    new_tokens = outputs[0, inputs["input_ids"].shape[-1]:]
    return tokenizer.decode(new_tokens, skip_special_tokens=True).strip()

# ------------------------ EXTRACTION HELPERS ------------------------

FENCE_BLOCK_RE = re.compile(r"```([^\n`]*)\n(.*?)```", re.DOTALL)

def _label_score(info: str) -> int:
    """Score how likely the fence info refers to vulnerable/fixed files."""
    info_l = info.lower()
    score = 0
    if "a1_vulnerable" in info_l: score += 3
    if "a1_fixed" in info_l:      score += 3
    if "vulnerable" in info_l:    score += 1
    if "fixed" in info_l:         score += 1
    if "rust" in info_l:          score += 1
    return score

def extract_variants(text: str) -> Tuple[str, str]:
    """
    Extract A1_vulnerable.rs and A1_fixed.rs blocks from model output.
    Strategy:
      1) Exact-named fences: ```A1_vulnerable.rs ...``` and ```A1_fixed.rs ...```
      2) Any fences: choose two best-scoring by labels / heuristics
      3) Otherwise, return ("","") to signal parse failure
    """
    # 1) Exact-named
    m_v = re.search(r"```A1_vulnerable\.rs\s*(.*?)```", text, re.DOTALL | re.IGNORECASE)
    m_f = re.search(r"```A1_fixed\.rs\s*(.*?)```", text, re.DOTALL | re.IGNORECASE)
    if m_v and m_f:
        return m_v.group(1).strip(), m_f.group(1).strip()

    # 2) Any fences
    fences = FENCE_BLOCK_RE.findall(text)
    if len(fences) >= 2:
        scored = [(idx, _label_score(info), info, body.strip()) for idx, (info, body) in enumerate(fences)]
        scored.sort(key=lambda x: (-x[1], x[0]))  # score desc, then original order
        body1 = scored[0][3]
        body2 = scored[1][3]
        # Heuristic: vulnerable first if body contains more "unsafe"/"unwrap"/"panic"
        v_hint1 = sum(h in body1 for h in ("unsafe", "unwrap(", "panic!"))
        v_hint2 = sum(h in body2 for h in ("unsafe", "unwrap(", "panic!"))
        if v_hint1 >= v_hint2:
            return body1, body2
        else:
            return body2, body1

    return "", ""

# ----------------------------- DISCOVERY ----------------------------

def find_candidate_dirs(root: Path) -> List[Path]:
    """Yield directories under root that contain any of fixed/vulnerable/test files."""
    cands = []
    for dirpath, _dirnames, filenames in os.walk(root):
        p = Path(dirpath)
        if any(name in filenames for name in ("fixed.rs", "vulnerable.rs", "demo_test.rs", "oracle.rs", "orcale.rs")):
            cands.append(p)
    cands.sort()
    return cands

# --------------------------- PROCESS ONE DIR ------------------------

def process_dir(in_dir: Path,
                in_root: Path,
                out_root: Path,
                tokenizer,
                model,
                tries: int,
                temp_start: float,
                temp_step: float,
                model_id: str,
                only_missing: bool = False) -> dict:
    fixed_p = in_dir / "fixed.rs"
    vuln_p  = in_dir / "vulnerable.rs"

    # Test file fallback
    if (in_dir / "demo_test.rs").exists():
        test_p = in_dir / "demo_test.rs"
    elif (in_dir / "oracle.rs").exists():
        test_p = in_dir / "oracle.rs"
    elif (in_dir / "orcale.rs").exists():
        test_p = in_dir / "orcale.rs"
    else:
        return {"dir": str(in_dir), "status": "skip", "reason": "no demo_test.rs/oracle.rs"}

    if not (fixed_p.exists() and vuln_p.exists()):
        return {"dir": str(in_dir), "status": "skip", "reason": "missing fixed.rs or vulnerable.rs"}

    rel = in_dir.relative_to(in_root)
    out_dir = out_root / rel
    out_dir.mkdir(parents=True, exist_ok=True)

    # If outputs already exist and only_missing, skip
    if only_missing and (out_dir / "A1_vulnerable.rs").exists() and (out_dir / "A1_fixed.rs").exists():
        return {"dir": str(in_dir), "status": "skip", "reason": "already has A1 files"}

    fixed_src = read_text_fallback(fixed_p)
    vuln_src  = read_text_fallback(vuln_p)
    test_src  = read_text_fallback(test_p)

    temp = temp_start
    for _attempt in range(1, tries + 1):
        raw = call_model(tokenizer, model, fixed_src, vuln_src, test_src,
                         temperature=temp, max_new_tokens=GEN_MAX_NEW_TOKENS)
        a_v, a_f = extract_variants(raw)
        if a_v and a_f:
            (out_dir / "A1_vulnerable.rs").write_text(a_v, encoding="utf-8")
            (out_dir / "A1_fixed.rs").write_text(a_f, encoding="utf-8")
            # Copy the test to unified name demo_test.rs
            (out_dir / "demo_test.rs").write_text(test_src, encoding="utf-8")
            return {"dir": str(in_dir), "status": "ok"}
        temp += temp_step

    return {"dir": str(in_dir), "status": "failed"}

# -------------------------------- MAIN ------------------------------

def main():
    parser = argparse.ArgumentParser(description="DeepSeek A1 data generation (no verification).")
    parser.add_argument("--input",  default=DEFAULT_INPUT,  help="Input root or CWE directory")
    parser.add_argument("--output", default=DEFAULT_OUTPUT, help="Output root (mirrors input)")
    parser.add_argument("--model",  default=DEFAULT_MODEL_ID, help="Hugging Face model id to use")
    parser.add_argument("--tries",  type=int, default=TRIES_PER_DIR, help="Max extraction attempts per directory")
    parser.add_argument("--temp",   type=float, default=TEMP_START, help="Initial temperature")
    parser.add_argument("--temp-step", type=float, default=TEMP_STEP, help="Temperature increment per retry")
    parser.add_argument("--only-missing", action="store_true", help="Skip dirs that already have A1 files")
    args = parser.parse_args()

    in_root  = Path(args.input).expanduser().resolve()
    out_root = Path(args.output).expanduser().resolve()
    out_root.mkdir(parents=True, exist_ok=True)

    print(f"[info] Input : {in_root}")
    print(f"[info] Output: {out_root}")
    print(f"[info] Model : {args.model}")

    # Load LLM once
    tok, mdl = load_model(args.model)

    # Iterate candidate dirs in sorted order
    candidates = find_candidate_dirs(in_root)
    if not candidates:
        print("[warn] No candidate directories found.")
        sys.exit(0)

    total_dirs = 0
    ok = failed = skipped = 0

    for d in candidates:
        has_fixed = (d / "fixed.rs").exists()
        has_vuln  = (d / "vulnerable.rs").exists()
        has_test  = any((d / name).exists() for name in ("demo_test.rs", "oracle.rs", "orcale.rs"))
        if not (has_fixed and has_vuln and has_test):
            continue

        total_dirs += 1
        rel = d.relative_to(in_root)
        print(f"[{total_dirs}] Generating A1 for: {rel}")

        summary = process_dir(
            in_dir=d,
            in_root=in_root,
            out_root=out_root,
            tokenizer=tok,
            model=mdl,
            tries=args.tries,
            temp_start=args.temp,
            temp_step=args.temp_step,
            model_id=args.model,
            only_missing=args.only_missing,
        )
        st = summary.get("status")
        if st == "ok":
            ok += 1
            print("    OK")
        elif st == "failed":
            failed += 1
            print("    FAILED")
        else:
            skipped += 1
            print(f"    SKIPPED ({summary.get('reason','')})")

    print("\n[summary]")
    print(f"  triplet dirs processed : {total_dirs}")
    print(f"  ok                     : {ok}")
    print(f"  failed                 : {failed}")
    print(f"  skipped                : {skipped}")
    print(f"  output root            : {out_root}")

if __name__ == "__main__":
    main()