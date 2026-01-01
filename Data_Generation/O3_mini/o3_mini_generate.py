import os
from openai import OpenAI

# ------------------------------
# PROMPT SETUP
# ------------------------------

CWE_DESCRIPTIONS = {
    "CWE-190": "Integer Overflow or Wraparound occurs when arithmetic exceeds the variable's max capacity. In Rust, this often occurs using unchecked math or unsafe typecasting."
}


SYSTEM_PROMPT = """
You are an expert in secure systems programming and vulnerability synthesis.
Generate complex and realistic Rust code that mimics real-world vulnerabilities from RustSec or CVE reports.
The output must be hard to detect and compile correctly with unsafe Rust constructs where needed.
Ensure CWE-specific realism by emulating real usage patterns and community idioms.
"""

PROMPT_TEMPLATE = """
### Instruction:
Generate a *complex*, *realistic*, and *compilable* vulnerable and fixed Rust code pair for {cwe}.
The output must include:


1. A vulnerable Rust code snippet in the tags <vulnerable> </vulnerable>.
2. A description of Vulnerable code <vuldesc> </vuldesc>
3. A corrected version that fixes the vulnerability in the tags <fixed> </fixed>.
4. A description of Fixed code <fixeddesc> </fixeddesc>
5. A JSON array of line numbers where the vulnerability occurs in the tags <lineno></lineno>.
6. A test oracle function that fails for the vulnerable version and passes for the fixed version in the tags <oracle></oracle>.

The vulnerable code should use unsafe blocks, concurrency, smart pointers, or lifetimes in ways that mimic real-world issues reported in RustSec.
Do not generate trivial code. Use structs, traits, multiple functions, and realistic APIs.

### Input:
{description}

### Response:
"""

# ------------------------------
# MODEL INITIALIZATION
# ------------------------------
print("[*] Loading model and tokenizer...")

# Read your key from env: export OPENAI_API_KEY=""
client = OpenAI(api_key='')

def build_user_prompt(cwe: str) -> str:
    desc = CWE_DESCRIPTIONS.get(cwe, "")
    return PROMPT_TEMPLATE.format(cwe=cwe, description=desc)

def get_completion(prompt: str, model: str = "o3-mini", timeout: int = 60) -> str | None:
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
        print(f"Error: {e}")
        return None

if __name__ == "__main__":
    # Example: generate CWE-190 vulnerable/fixed pair
    user_prompt = build_user_prompt("CWE-190")
    output = get_completion(user_prompt, model="o3-mini", timeout=120)
    print(output if output is not None else "[!] No output returned.")
