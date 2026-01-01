#!/usr/bin/env python3
"""
A1.MakeVulnerable.py (Qwen 2.5 Coder 14B edition)
Generate ONLY A1_vulnerable.rs (introduce one realistic bug into fixed.rs so tests FAIL).

- Reads per-project pair: fixed.rs + test file (demo_test.rs/oracle.rs/orcale.rs).
- Optionally reads vulnerable.rs (reference only; DO NOT copy).
- Writes to mirrored OUTPUT dir:
    * A1_vulnerable.rs  (generated)
    * demo_test.rs      (copied from input test file, regardless of original name)
- No verification compile/run here; just generation.
- Test-aware prompting: we feed the model a compact summary of public symbols
  referenced by unit tests so the injected bug actually trips those tests.

Recommended environment (CUDA 12.x OK):
    pip install -U "transformers>=4.44.2" "accelerate>=0.33.0" "torch>=2.1"

If your GPU RAM is limited, you can run on CPU by forcing CUDA visibility:
    CUDA_VISIBLE_DEVICES= python3 A1.MakeVulnerable.py ...

"""

import os
import re
import sys
import argparse
from pathlib import Path
from typing import List

# keep BLAS/OpenMP libs from spawning many threads
#os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
#os.environ.setdefault("OMP_NUM_THREADS", "1")
#os.environ.setdefault("MKL_NUM_THREADS", "1")
#os.environ.setdefault("NUMEXPR_NUM_THREADS", "1")
#os.environ.setdefault("TOKENIZERS_PARALLELISM", "false")

# ---------------------- DEFAULT PATHS & SETTINGS ----------------------

DEFAULT_INPUT  = "/home/ikqnm/PycharmProjects/PythonProject/DataGenerated_pairs/CWE-665"
DEFAULT_OUTPUT = "/home/ikqnm/PycharmProjects/PythonProject/deepseek/DataAugmented_pairs_new/CWE-665"

# Switch to Qwen2.5 Coder 14B Instruct by default
DEFAULT_MODEL_ID = "Qwen/Qwen2.5-Coder-14B-Instruct"

# Generation setup
GEN_MAX_NEW_TOKENS = 6000
# Randomize generation hyperparameters each run:
# - TEMP_START: initial sampling temperature (0.2 .. 1.0)
# - TEMP_STEP: incremental temperature step per retry (0.05 .. 0.30)
# - SEED: random integer seed for deterministic model APIs (if used)
import random as _random
_sys_rand = _random.SystemRandom()
TEMP_START = round(_sys_rand.uniform(0.1, 1.0), 3)
TEMP_STEP  = round(_sys_rand.uniform(0.05, 0.30), 3)
TRIES_PER_DIR      = 6
SEED               = _sys_rand.randint(0, 2**31 - 1)

# ----------------------------- CWE HINTS -----------------------------
# Keep these crisp & prescriptive so the model chooses a bug pattern that breaks tests.
CWE_HINTS = {
    "CWE-665": (
        "CWE-665: Improper Initialization: using data before fully initialized. In Rust, MaybeUninit misuse, set_len without writes, or missing default fields."
        "Do not change any public function signatures; keep changes minimal. produce a vulnerable sample of the code."
    ),
}

# ----------------------------- PROMPTS ------------------------------

SYSTEM_MSG = (
    "You are an expert Rust engineer. Produce minimal and compilable edits.\n"
    "Keep ALL public function names, signatures, visibility, and return types unchanged—especially anything used by the unit tests.\n"
    "No new crates; standard library only. Minimize edits and keep the file compiling.\n"
)

USER_TEMPLATE = """\
We have a Rust project with these files in the same module directory:
- fixed.rs       (currently passes unit tests)
- demo_test.rs   (unit tests; DO NOT edit)
- vulnerable.rs  (OPTIONAL; reference only — DO NOT copy code verbatim)

CWE requirement: {cwe_code}
Guidance: {cwe_guidance}

Unit‑Test Awareness (reference only; DO NOT edit tests):
- The tests directly/indirectly reference these public symbols and items (names, signatures, attributes). Do not rename or remove them.
{test_summary}

Important note about vulnerable.rs:
- If vulnerable.rs is provided, you may consult it ONLY to infer patterns.
- DO NOT copy code from vulnerable.rs. Inject a FOCUSED edit into fixed.rs, aligned with {cwe_code}.

Your task:
Create **A1_vulnerable.rs** by introducing small, realistic bug into fixed.rs so that the existing unit tests FAIL.
The bug MUST align with {cwe_code}. Keep the code compiling. Avoid large refactors and keep diff minimal. Check {fixed_src} and take use it an example code but don't copy from it.

Rules:
- DO NOT change any public item names/signatures/visibility referenced by tests.
- Keep behavior change minimal; inject one focused defect only.
- Standard library only; no new extern crates.
- No extra test code, no comments about the bug rationale—just the full modified source.

FORMAT (STRICT):
- Reply with EXACTLY one fenced code block with the filename on the fence line:
```A1_vulnerable.rs
<entire file here>
```

--- fixed.rs ---
{fixed_src}

--- demo_test.rs ---
{test_src}

--- vulnerable.rs (reference; may be empty) ---
{vuln_src}
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
    # Prefer bfloat16 on GPU; float32 on CPU
    try:
        import torch
        use_gpu = torch.cuda.is_available()
        dtype = torch.bfloat16 if use_gpu else torch.float32
    except Exception:
        dtype = None

    # Qwen 2.5 models ship with chat templates; trust_remote_code is safe here.
    tok = AutoTokenizer.from_pretrained(model_id, trust_remote_code=True)
    mdl = AutoModelForCausalLM.from_pretrained(
        model_id,
        torch_dtype=dtype if dtype is not None else None,
        device_map="auto" if ("torch" in sys.modules and getattr(__import__("torch"), "cuda", None) and __import__("torch").cuda.is_available()) else None,
        trust_remote_code=True,
    )
    return tok, mdl

def apply_chat_template(tokenizer, messages) -> str:
    # For Qwen2.5, this produces the correct chat-format prompt
    try:
        return tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    except Exception:
        # Fallback generic chat format
        text = ""
        for m in messages:
            role = m.get("role", "user")
            content = m.get("content", "")
            text += f"<|{role}|>\n{content}\n"
        text += "<|assistant|>\n"
        return text

def call_model(tokenizer, model, fixed_src: str, test_src: str, vuln_src: str,
               cwe_code: str, cwe_guidance: str, test_summary: str,
               temperature: float, max_new_tokens: int,
               do_sample: bool = True) -> str:
    messages = [
        {"role": "system", "content": SYSTEM_MSG},
        {"role": "user",   "content": USER_TEMPLATE.format(
            fixed_src=fixed_src,
            test_src=test_src,
            vuln_src=vuln_src,
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
    # prune Nones so HF doesn't warn
    gen_kwargs = {k: v for k, v in gen_kwargs.items() if v is not None}
    outputs = model.generate(**inputs, **gen_kwargs)
    new_tokens = outputs[0, inputs["input_ids"].shape[-1]:]
    return tokenizer.decode(new_tokens, skip_special_tokens=True).strip()

# ------------------------ EXTRACTION HELPERS ------------------------

FENCE_BLOCK_RE = re.compile(r"```([^\n`]*)\n(.*?)```", re.DOTALL)

def extract_vulnerable(text: str) -> str:
    # 1) Exact-named
    m = re.search(r"```A1_vulnerable\.rs\s*(.*?)```", text, re.DOTALL | re.IGNORECASE)
    if m:
        return m.group(1).strip()
    # 2) Any fence — fallback
    m2 = re.search(r"```[^\n`]*\n(.*?)```", text, re.DOTALL)
    return m2.group(1).strip() if m2 else ""

# ---------------------- TEST SIGNATURE SUMMARIZER -------------------

TYPE_OR_STRUCT = re.compile(r"(?m)^\s*pub\s+(?:struct|enum|trait)\s+([A-Za-z_]\w*)")
USE_ITEM       = re.compile(r"(?m)^\s*pub\s+use\s+(.+);")

def summarize_test_targets(test_src: str) -> str:
    """
    Heuristically extract public-facing names referenced by tests:
      - function calls (foo(, module::foo(), Type::method())
      - explicit `pub struct/enum/trait` in tests (rare)
      - `use` re-exports
    We return a compact bullet list to steer the LLM.
    """
    # Identify likely calls/paths in assertions & test code
    called = set(re.findall(r"(?m)(?:^|[^A-Za-z0-9_])([A-Za-z_]\w*)\s*\(", test_src))
    # Drop common test macros and std items
    blacklist = {"assert", "assert_eq", "assert_ne", "println", "format", "panic", "dbg", "assert_matches"}
    called = {c for c in called if c not in blacklist}

    types = set(TYPE_OR_STRUCT.findall(test_src))
    uses  = set(USE_ITEM.findall(test_src))

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

# ----------------------------- DISCOVERY ----------------------------

def last_7_cwe(path: Path) -> str:
    s = str(path)
    return s[-7:] if len(s) >= 7 else s

def cwe_guidance_for(code: str) -> str:
    return CWE_HINTS.get(code, f"{code}: Inject exactly one minimal, realistic bug consistent with this CWE while keeping the file compiling.")

def find_candidate_dirs(root: Path) -> List[Path]:
    cands = []
    for dirpath, _dirnames, filenames in os.walk(root):
        if "fixed.rs" in filenames and any(n in filenames for n in ("demo_test.rs", "oracle.rs", "orcale.rs")):
            cands.append(Path(dirpath))
    cands.sort()
    return cands

# --------------------------- PROCESS ONE DIR ------------------------

def process_dir(d: Path, in_root: Path, out_root: Path, tok, mdl,
                tries: int, t0: float, tstep: float, only_missing: bool,
                cwe_code: str, cwe_desc: str, debug: bool = False) -> dict:
    fixed_p = d / "fixed.rs"
    if (d / "demo_test.rs").exists():
        test_p = d / "demo_test.rs"
    elif (d / "oracle.rs").exists():
        test_p = d / "oracle.rs"
    elif (d / "orcale.rs").exists():
        test_p = d / "orcale.rs"
    else:
        return {"dir": str(d), "status": "skip", "reason": "no test file"}

    # optional vulnerable.rs as a reference
    vuln_p = d / "vulnerable.rs"
    vuln_src = read_text_fallback(vuln_p) if vuln_p.exists() else ""

    rel = d.relative_to(in_root)
    out_dir = out_root / rel
    out_dir.mkdir(parents=True, exist_ok=True)

    if only_missing and (out_dir / "A1_vulnerable.rs").exists():
        return {"dir": str(d), "status": "skip", "reason": "existing A1_vulnerable.rs"}

    fixed_src = read_text_fallback(fixed_p)
    test_src  = read_text_fallback(test_p)
    test_summary = summarize_test_targets(test_src)

    temp = t0

    # Attempt 1: deterministic (no sampling) to stabilize format
    resp = call_model(
        tok, mdl,
        fixed_src=fixed_src, test_src=test_src, vuln_src=vuln_src,
        cwe_code=cwe_code, cwe_guidance=cwe_desc, test_summary=test_summary,
        temperature=temp, max_new_tokens=GEN_MAX_NEW_TOKENS,
        do_sample=False
    )
    body = extract_vulnerable(resp)
    if body:
        (out_dir / "A1_vulnerable.rs").write_text(body, encoding="utf-8")
        (out_dir / "demo_test.rs").write_text(test_src, encoding="utf-8")
        return {"dir": str(d), "status": "ok"}
    if debug:
        (out_dir / "A1_raw_attempt1.txt").write_text(resp, encoding="utf-8")
        print("----- RAW OUTPUT (attempt 1) BEGIN -----")
        print(resp[:2000])
        print("----- RAW OUTPUT (attempt 1) END -----")

    # Subsequent attempts: sampling with temperature ramp
    for attempt in range(2, tries + 1):
        resp = call_model(
            tok, mdl,
            fixed_src=fixed_src, test_src=test_src, vuln_src=vuln_src,
            cwe_code=cwe_code, cwe_guidance=cwe_desc, test_summary=test_summary,
            temperature=temp, max_new_tokens=GEN_MAX_NEW_TOKENS,
            do_sample=True
        )
        body = extract_vulnerable(resp)
        if body:
            (out_dir / "A1_vulnerable.rs").write_text(body, encoding="utf-8")
            (out_dir / "demo_test.rs").write_text(test_src, encoding="utf-8")
            return {"dir": str(d), "status": "ok"}
        if debug:
            (out_dir / f"A1_raw_attempt{attempt}.txt").write_text(resp, encoding="utf-8")
            print(f"----- RAW OUTPUT (attempt {attempt}) BEGIN -----")
            print(resp[:2000])
            print(f"----- RAW OUTPUT (attempt {attempt}) END -----")
        temp += tstep

    return {"dir": str(d), "status": "failed"}

# -------------------------------- MAIN ------------------------------

def main():
    ap = argparse.ArgumentParser(description="Generate A1_vulnerable.rs from fixed.rs + tests (CWE-constrained, test-aware prompt) using Qwen2.5 Coder 14B.")
    ap.add_argument("--input",  default=DEFAULT_INPUT,  help="Input root or CWE directory (last 7 chars used as CWE code)")
    ap.add_argument("--output", default=DEFAULT_OUTPUT, help="Output root (mirrors input)")
    ap.add_argument("--model",  default=DEFAULT_MODEL_ID, help="Hugging Face model id")
    ap.add_argument("--tries",  type=int, default=TRIES_PER_DIR, help="Max attempts per directory")
    ap.add_argument("--temp",   type=float, default=TEMP_START, help="Initial temperature")
    ap.add_argument("--temp-step", type=float, default=TEMP_STEP, help="Temperature step per retry")
    ap.add_argument("--only-missing", action="store_true", help="Skip dirs that already have A1_vulnerable.rs")
    ap.add_argument("--debug", action="store_true", help="Dump raw model outputs on failure and print first 2000 chars")
    args = ap.parse_args()

    in_root  = Path(args.input).expanduser().resolve()
    out_root = Path(args.output).expanduser().resolve()
    out_root.mkdir(parents=True, exist_ok=True)

    # Derive CWE from last 7 characters of input path
    cwe_code = last_7_cwe(in_root)
    cwe_desc = cwe_guidance_for(cwe_code)

    print(f"[info] Input : {in_root}")
    print(f"[info] Output: {out_root}")
    print(f"[info] Model : {args.model}")
    print(f"[info] CWE   : {cwe_code}")

    tok, mdl = load_model(args.model)
    dirs = find_candidate_dirs(in_root)
    if not dirs:
        print("[warn] No candidate directories found."); sys.exit(0)

    total = ok = failed = skipped = 0
    for d in dirs:
        total += 1
        rel = d.relative_to(in_root)
        print(f"[{total}] A1_vulnerable for: {rel}")
        res = process_dir(
            d, in_root, out_root, tok, mdl,
            args.tries, args.temp, args.temp_step, args.only_missing,
            cwe_code=cwe_code, cwe_desc=cwe_desc, debug=args.debug
        )
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