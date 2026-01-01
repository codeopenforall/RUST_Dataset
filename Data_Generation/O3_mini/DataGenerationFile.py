import os
import re
import json
from pathlib import Path
from typing import Optional
from openai import OpenAI

# ------------------------------
# CONFIG
# ------------------------------
PAIRS_PER_CWE = 20  # generate N pairs per CWE

CWE_LIST = [
    "CWE-662"
]

CWE_DESCRIPTIONS_2 = {
    # (kept as-is; not used in this snippet)
}

CWE_DESCRIPTIONS = {
    "CWE-662": "Improper Synchronization: incorrect locking/ordering causing races/deadlocks. In Rust, mixing Arc/Mutex/RwLock incorrectly or lock poisoning misuse."
}

SYSTEM_PROMPT = """
You are an expert in secure systems programming and vulnerability synthesis.
Generate complex and realistic Rust code that mimics real-world vulnerabilities from RustSec or CVE reports.
The output must be compile correctly with unsafe Rust constructs where needed.
Ensure CWE-specific realism by emulating real usage patterns and community idioms.

Note: You should not include the oracle code or test code in vulnerable and fixed code.
"""

PROMPT_TEMPLATE = """
### Instruction:
Generate a *Super complex*, *realistic*, *compilable* and *testable* vulnerable and fixed Rust code pair for {cwe} against a unittest.
The output must include:

1. A vulnerable Rust code snippet in the tags <vulnerable> </vulnerable>.
2. A description of Vulnerable code <vuldesc> </vuldesc>
3. A corrected version that fixes the vulnerability in the tags <fixed> </fixed>.
4. A description of Fixed code <fixeddesc> </fixeddesc>
5. A JSON array of line numbers where the vulnerability occurs in the tags <lineno></lineno>.
6. A test oracle function that fails for the vulnerable version and passes for the fixed version in the tags <oracle></oracle>.

The vulnerable code should use unsafe blocks, concurrency, smart pointers, or lifetimes in ways that mimic real-world issues reported in RustSec.
Do not generate trivial code. Use structs, traits, multiple functions, and realistic APIs.
The most important the code should be compilable, and oracle should be separate from fixed and vulnerable code.
Do not use vulnerable or fixed word in the generated code at any level int function name variable name etc..,.

The vulnerable and fixed code should have a main method.

NOTE: YOU SHOULD NOT INCLUDE THE ORACLE CODE OR TEST CODE IN VULNERABLE AND FIXED CODE.

Quick sanity checklist
One test, one contract. (Fixed passes, vulnerable fails.)
Triggering input included. (E.g., boundary values, extremes, malformed.)
Explicit assertions. (Don’t rely on prints; assert Result, value, or invariant.)
Reproducible. (Seeded RNG, deterministic concurrency, no timing assumptions.)
Follow this and you’ll have execution results that prove the vulnerable code is actually vulnerable (test fails) and the fixed code is genuinely fixed (test passes)—without changing the test between runs.


### Input:
{description}

### Response:
"""

# ------------------------------
# MODEL INITIALIZATION
# ------------------------------
print("[*] Loading model and tokenizer...")

# Prefer setting key via env: export OPENAI_API_KEY="..."
client = OpenAI(
    #api_key='')
     api_key='')


def build_user_prompt(cwe: str) -> str:
    desc = CWE_DESCRIPTIONS.get(cwe, "")
    return PROMPT_TEMPLATE.format(cwe=cwe, description=desc)


def get_completion(prompt: str, model: str = "o3-mini", timeout: int = 120) -> Optional[str]:
    try:
        resp = client.with_options(timeout=timeout).chat.completions.create(
            model=model,
            messages=[
                {"role": "system", "content": SYSTEM_PROMPT.strip()},
                {"role": "user", "content": prompt},
            ],
            top_p=1.0,
            n=1,
        )
        return resp.choices[0].message.content
    except Exception as e:
        print(f"[!] Error: {e}")
        return None


# ------------------------------
# PARSING HELPERS
# ------------------------------
TAG_PATTERNS = {
    "vulnerable": re.compile(r"<vulnerable>(.*?)</vulnerable>", re.DOTALL | re.IGNORECASE),
    "vuldesc": re.compile(r"<vuldesc>(.*?)</vuldesc>", re.DOTALL | re.IGNORECASE),
    "fixed": re.compile(r"<fixed>(.*?)</fixed>", re.DOTALL | re.IGNORECASE),
    "fixeddesc": re.compile(r"<fixeddesc>(.*?)</fixeddesc>", re.DOTALL | re.IGNORECASE),
    "lineno": re.compile(r"<lineno>(.*?)</lineno>", re.DOTALL | re.IGNORECASE),
    "oracle": re.compile(r"<oracle>(.*?)</(?:orcale|oracle)>", re.DOTALL | re.IGNORECASE),
}


def extract_tag(text: str, key: str) -> str:
    pat = TAG_PATTERNS[key]
    m = pat.search(text or "")
    return m.group(1).strip() if m else ""


def ensure_json_array(s: str) -> list:
    try:
        v = json.loads(s)
        return v if isinstance(v, list) else [v]
    except Exception:
        nums = re.findall(r"\d+", s or "")
        return [int(n) for n in nums] if nums else []


# ------------------------------
# FILE WRITING
# ------------------------------
def write_pair(base_dir: Path, pair_idx: int, content: str):
    pair_dir = base_dir / f"pair{pair_idx}"
    pair_dir.mkdir(parents=True, exist_ok=True)

    vulnerable = extract_tag(content, "vulnerable")
    vuldesc = extract_tag(content, "vuldesc")
    fixed = extract_tag(content, "fixed")
    fixeddesc = extract_tag(content, "fixeddesc")
    lineno = extract_tag(content, "lineno")
    oracle = extract_tag(content, "oracle")

    lineno_arr = ensure_json_array(lineno)
    lineno_js = (
        "// Autogenerated\n"
        f"const lineno = {json.dumps(lineno_arr, indent=2)};\n"
        "module.exports = lineno;\n"
    )

    (pair_dir / "vulnerable.rs").write_text(vulnerable, encoding="utf-8")
    (pair_dir / "vuldesc.txt").write_text(vuldesc, encoding="utf-8")
    (pair_dir / "fixed.rs").write_text(fixed, encoding="utf-8")
    (pair_dir / "fixeddesc.txt").write_text(fixeddesc, encoding="utf-8")
    (pair_dir / "lineno.js").write_text(lineno_js, encoding="utf-8")
    (pair_dir / "demo_test.rs").write_text(oracle, encoding="utf-8")

    print(f"[+] Wrote {pair_dir}/{{vulnerable.rs,vuldesc.txt,fixed.rs,fixeddesc.txt,lineno.js,demo_test.rs}}")


# ------------------------------
# MAIN
# ------------------------------
if __name__ == "__main__":
    output_root = Path("DataGenerated_pairs_new")
    output_root.mkdir(exist_ok=True)

    for cwe in CWE_LIST:
        cwe_dir = output_root / cwe
        cwe_dir.mkdir(parents=True, exist_ok=True)
        print(f"[*] Generating up to {PAIRS_PER_CWE} pairs for {cwe}...")

        for i in range(1, PAIRS_PER_CWE + 1):  # pair1..pairN
            pair_dir = cwe_dir / f"pair{i}"

            # >>> NEW: skip existing pair directory <<<
            if pair_dir.exists():
                print(f"  [skip] {cwe}/pair{i} already exists; skipping generation.")
                continue

            print(f"  [gen ] {cwe} -> pair{i}")
            prompt = build_user_prompt(cwe)
            resp = get_completion(prompt, model="o3-mini", timeout=120)

            if not resp:
                print(f"  [!] No output for {cwe}/pair{i}; skipping.")
                continue

            write_pair(cwe_dir, i, resp)

    print("[*] Done.")
