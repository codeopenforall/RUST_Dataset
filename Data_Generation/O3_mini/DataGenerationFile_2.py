import os
import re
import json
from pathlib import Path
from typing import Optional
from openai import OpenAI

# ------------------------------
# CONFIG
# ------------------------------
PAIRS_PER_CWE = 20  # generate 4 pairs per CWE

CWE_LIST = [
    "CWE-190"
]

CWE_DESCRIPTIONS_2 = {
    #"CWE-416": "Use-After-Free: using memory after it was freed/dropped. In Rust this can appear in unsafe code, FFI, or aliasing violations with raw pointers/Rc<RefCell> cycles.",
    #"CWE-125": "Out-of-Bounds Read: reading past the end/before the start of a buffer. In Rust, unchecked pointer arithmetic, slice::get_unchecked, or FFI can trigger it.",
    #"CWE-400": "Uncontrolled Resource Consumption (DoS): unbounded CPU/memory/file handles. In Rust, accept() loops without backpressure, unbounded channels, or Vec growth can exhaust resources.",
    #"CWE-787": "Out-of-Bounds Write: writing outside buffer bounds. In Rust, unsafe pointer writes, set_len misuse, or unchecked indexing in unsafe blocks cause memory corruption.",
    #"CWE-079": "Cross-Site Scripting (XSS): reflecting untrusted input into HTML/JS without escaping. In Rust web stacks, naive format!/push_str into templates leads to XSS.",
    #"CWE-119": "Improper Restriction of Operations within Memory Bounds: generic memory safety errors. In Rust, most often from unsafe blocks/FFI breaking aliasing or bounds.",
    #"CWE-020": "Improper Input Validation: failing to validate user-controlled data (sizes, ranges, enums). In Rust, unchecked parse(), from_utf8_unchecked, or trusting lengths is risky.",
    #"CWE-190": "Integer Overflow/Wraparound: arithmetic exceeds type capacity. In Rust, unchecked ops in release mode or cast truncation (as) can wrap and corrupt logic.",
    #"CWE-022": "Path Traversal: using ../ or absolute paths to escape intended dirs. In Rust, naive PathBuf::join without canonicalization / prefix checks is abusable.",
    #"CWE-203": "Observable Discrepancy: error messages/timing reveal sensitive state. In Rust, differing Result errors or early returns can leak ‘existence’ or auth info.",
    #"CWE-347": "Improper Verification of Cryptographic Signature: accepting data without properly verifying signatures/algorithms. In Rust, misuse of ring/ed25519-dalek APIs or skipping context/domain checks.",
    #"CWE-682": "Incorrect Calculation: logic/math errors (precedence, rounding, units). In Rust, integer division, overflowed intermediates, or wrong time/size units.",
    #"CWE-908": "Use of Uninitialized Resource: object/config/file used before it is fully initialized. In Rust, uninitialized memory via MaybeUninit misuse or default placeholders not set.",
    #"CWE-059": "Failure to properly validate or restrict symbolic link resolution, allowing attackers to access or modify unintended files.",
    #"CWE-193": "Off-by-One Error: fencepost mistakes in indices/lengths. In Rust, len vs last index, set_len(n) with n off by one, or slicing mistakes.",
    #"CWE-252": "Unchecked Return Value: ignoring Results that indicate failure. In Rust, dropping a Result (let _ = …) or using .ok() and proceeding anyway.",
    #"CWE-415": "Double Free: freeing the same memory twice. In Rust, unsafe manual deallocation, multiple Box from same pointer, or incorrect FFI ownership.",
    #"CWE-427": "Uncontrolled Search Path Element: allowing attacker-controlled PATH/LD_LIBRARY_PATH/DYLD_* to influence loading. In Rust, spawning commands that trust env PATH.",
    #"CWE-444": "Inconsistent Interpretation of HTTP Requests (Request Smuggling): proxy/backend parse differences. In Rust, mismatched hyper/actix header parsing vs proxy.",
    #"CWE-670": "Always-Incorrect Control Flow Implementation: flawed state/branch logic that is never correct. In Rust, incorrect match arms, break/continue/error paths causing invariant violations.",
    #"CWE-674": "Uncontrolled Recursion: recursion without depth limits leading to stack overflow. In Rust, recursive descent parsing or data walkers on attacker input.",
    #"CWE-754": "Improper Check for Exceptional Conditions: checking wrong flags/states. In Rust, matching on the wrong error kind or assuming Ok after timeout/cancel.",
    #"CWE-755": "Improper Handling of Exceptional Conditions: swallowing/losing errors. In Rust, .unwrap_or_default() or .expect(\"ignore\") hiding real failures.",
    #"CWE-770": "Allocation of Resources Without Limits or Throttling: missing caps on memory/threads/files. In Rust, unbounded async tasks/channels or huge Vec reserve.",
    #"CWE-863": "Incorrect Authorization: permissions not enforced or enforced on wrong object. In Rust services, role checks after side effects or trusting client claims.",
    #"CWE-088": "OS Command Injection: passing untrusted strings to shells. In Rust, using sh -c or Command with .arg(format!(…)) that injects metacharacters.",
    #"CWE-113": "HTTP Response Splitting: CRLF injection into headers. In Rust, writing headers from untrusted input without stripping \\r\\n.",
    #"CWE-131": "Incorrect Calculation of Buffer Size: size computations ignore terminator/struct padding. In Rust, misusing size_of vs size_of_val or UTF-8 byte length.",
    #"CWE-134": "Use of Externally-Controlled Format String: format string taken from user. In Rust, format!(user_input, …) or write!(…, user_input).",
    #"CWE-191": "Integer Underflow (Wraparound): going below type’s min. In Rust, subtracting without checks or casting negative to unsigned.",
    #"CWE-200": "Exposure of Sensitive Information: leaking secrets via logs, panics, or error texts. In Rust, Debug/Display of secrets, backtraces, or default serde dumps.",
    #"CWE-248": "Uncaught Exception: abnormal termination without handling. In Rust, panics propagating across FFI/threads or not using catch_unwind where needed.",
    #"CWE-253": "Incorrect Check of Function Return Value: misinterpreting success/failure. In Rust, treating Err as success or ignoring Option::None cases.",
    #"CWE-276": "Incorrect Default Permissions: overly permissive files/sockets. In Rust, create/open without mode restrictions or serving dirs with 0o777.",
    #"CWE-285": "Improper Authorization: missing or weak access controls. In Rust APIs, endpoints callable without verifying caller privileges or resource ownership.",
    #"CWE-288": "Authentication Bypass Using an Alternate Path: debug endpoints/headers or alternate flows that skip auth. In Rust, feature flags/test routes left enabled.",
    #"CWE-311": "Missing Encryption of Sensitive Data: storing/transmitting secrets in clear. In Rust, plaintext config/env over HTTP or unencrypted at rest.",
    #"CWE-346": "Origin Validation Error: trusting Host/Origin/Referer blindly. In Rust, lax CORS or reverse proxy headers allowing CSRF or SSRF pivots.",
    #"CWE-362": "Race Condition: time-dependent bugs in concurrent code. In Rust, shared state without synchronization, TOCTOU, or non-atomic flags.",
    #"CWE-369": "Divide by Zero: arithmetic with zero divisors. In Rust, unchecked / or % with user input; panics or UB in unsafe numeric code.",
    #"CWE-426": "Untrusted Search Path: executable/library loaded from unsafe locations. In Rust, Command::new without absolute path; DLL preloading risk.",
    #"CWE-475": "Undefined Behavior for Input to API: violating API preconditions. In Rust, passing invalid pointers/sizes to unsafe/FFI functions.",
    #"CWE-611": "XXE: Improper Restriction of XML External Entities. In Rust, XML parsers allowing external entity resolution / file/HTTP fetch.",
    #"CWE-617": "Reachable Assertion: assert!() on attacker-influenced state triggers DoS. In Rust, debug assertions compiled in production or unwrap() on tainted data.",
    #"CWE-662": "Improper Synchronization: incorrect locking/ordering causing races/deadlocks. In Rust, mixing Arc/Mutex/RwLock incorrectly or lock poisoning misuse.",
    "CWE-665": "Improper Initialization: using data before fully initialized. In Rust, MaybeUninit misuse, set_len without writes, or missing default fields.",
    #"CWE-668": "Exposure of Resource to Wrong Sphere: exposing internals to wider context. In Rust, publishing sensitive files/socket on public interfaces.",
    #"CWE-701": "Incorrect Handling of Data Types/Boundaries Across Components: cross-boundary assumptions fail. In Rust, FFI/type conversions that mismatch sizes/endianness.",
    #"CWE-703": "Improper Check or Handling of Exceptional Conditions: broadly mishandling errors. In Rust, blanket .unwrap()/.expect(), ignoring JoinError/RecvError.",
    #"CWE-758": "Reliance on Undefined/Unspecified/Implementation-Defined Behavior: code depends on compiler/platform quirks. In Rust, UB in unsafe, layout assumptions, or unspecified iteration order.",
    #"CWE-824": "Access of Uninitialized Pointer: dereferencing before initialization. In Rust, reading from MaybeUninit or raw pointers prior to write.",
    #"CWE-835": "Loop with Unreachable Exit (Infinite Loop): missing termination. In Rust, while true without break based on attacker-controlled condition."
}

CWE_DESCRIPTIONS = {
"CWE-190": "Integer Overflow/Wraparound: arithmetic exceeds type capacity. In Rust, unchecked ops in release mode or cast truncation (as) can wrap and corrupt logic.",
}


SYSTEM_PROMPT = """
You are an expert in secure systems programming and vulnerability synthesis.
Generate complex and realistic Rust code that mimics real-world vulnerabilities from RustSec or CVE reports.
The output must be compile correctly with unsafe Rust constructs where needed.
Ensure CWE-specific realism by emulating real usage patterns and community idioms.

Ensure CWE-specific realism by emulating real usage patterns and community idioms.
"""

PROMPT_TEMPLATE = """
### Instruction:
Generate a *complex*, *realistic*, *compilable* and *testable* vulnerable and fixed Rust code pair for {cwe} against a unittest.
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
The vulnerable and fixed code should not contain and text code.
The vulnerable and fixed code should have a main method.

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
client = OpenAI(api_key='')  # reads key from env if available

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
    vuldesc   = extract_tag(content, "vuldesc")
    fixed     = extract_tag(content, "fixed")
    fixeddesc = extract_tag(content, "fixeddesc")
    lineno    = extract_tag(content, "lineno")
    oracle    = extract_tag(content, "oracle")

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
    (pair_dir / "oracle.rs").write_text(oracle, encoding="utf-8")

    print(f"[+] Wrote {pair_dir}/{{vulnerable.rs,vuldesc.txt,fixed.rs,fixeddesc.txt,lineno.js,oracle.rs}}")

# ------------------------------
# MAIN
# ------------------------------
if __name__ == "__main__":
    output_root = Path("DataGenerated_pairs_new")
    output_root.mkdir(exist_ok=True)

    for cwe in CWE_LIST:
        cwe_dir = output_root / cwe
        cwe_dir.mkdir(parents=True, exist_ok=True)
        print(f"[*] Generating {PAIRS_PER_CWE} pairs for {cwe}...")

        for i in range(1, PAIRS_PER_CWE + 1):  # pair1..pair4
            print(f"  [*] {cwe} -> pair{i}")
            prompt = build_user_prompt(cwe)
            resp = get_completion(prompt, model="o3-mini", timeout=120)

            if not resp:
                print(f"  [!] No output for {cwe}/pair{i}; skipping.")
                continue

            write_pair(cwe_dir, i, resp)

    print("[*] Done.")
