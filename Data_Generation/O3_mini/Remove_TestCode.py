#!/usr/bin/env python3
import os
import sys
from pathlib import Path
from typing import Tuple, Dict

# Root folder to scan (fixed path as requested)
ROOT = Path("/home/ikqnm/PycharmProjects/rudra_sandbox/rudra_big/rudra_big/src/dataset/o3_mini/data_generation/CWE-134")

TARGET_FILENAMES = {"fixed.rs", "vulnerable.rs"}

def read_text(p: Path) -> str:
    try:
        return p.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return p.read_text(encoding="latin-1")

def write_text(p: Path, s: str) -> None:
    p.write_text(s, encoding="utf-8")

def _skip_ws_comments(s: str, i: int) -> int:
    n = len(s)
    while i < n:
        c = s[i]
        if c in " \t\r\n":
            i += 1
            continue
        if i + 1 < n and s[i] == "/" and s[i+1] == "/":  # //
            i += 2
            while i < n and s[i] != "\n":
                i += 1
            continue
        if i + 1 < n and s[i] == "/" and s[i+1] == "*":  # /* ... */
            i += 2
            depth = 1
            while i < n and depth > 0:
                if i + 1 < n and s[i] == "/" and s[i+1] == "*":
                    depth += 1; i += 2
                elif i + 1 < n and s[i] == "*" and s[i+1] == "/":
                    depth -= 1; i += 2
                else:
                    i += 1
            continue
        break
    return i

def _find_matching_brace(s: str, i: int) -> int:
    assert s[i] == "{"
    n = len(s)
    depth = 0
    j = i
    in_str = in_char = False
    in_line_cmt = False
    in_blk_cmt = 0

    while j < n:
        ch = s[j]
        ch2 = s[j:j+2]

        if in_line_cmt:
            if ch == "\n": in_line_cmt = False
            j += 1; continue
        if in_blk_cmt:
            if ch2 == "/*": in_blk_cmt += 1; j += 2; continue
            if ch2 == "*/": in_blk_cmt -= 1; j += 2; continue
            j += 1; continue
        if in_str:
            if ch == "\\": j += 2; continue
            if ch == '"': in_str = False
            j += 1; continue
        if in_char:
            if ch == "\\": j += 2; continue
            if ch == "'": in_char = False
            j += 1; continue

        if ch2 == "//": in_line_cmt = True; j += 2; continue
        if ch2 == "/*": in_blk_cmt = 1; j += 2; continue
        if ch == '"': in_str = True; j += 1; continue
        if ch == "'": in_char = True; j += 1; continue

        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return j
        j += 1
    return -1

def _strip_block(s: str, start: int, end_inclusive: int) -> str:
    return s[:start] + s[end_inclusive+1:]

def _looks_like_cfg_test_attr(s: str, start: int) -> Tuple[bool, int]:
    i = start
    if not (i < len(s) and s[i] == "#"): return (False, start)
    i += 1
    i = _skip_ws_comments(s, i)
    if i >= len(s) or s[i] != "[": return (False, start)
    i += 1
    i = _skip_ws_comments(s, i)

    ident_start = i
    while i < len(s) and (s[i].isalnum() or s[i] in "::_"):
        i += 1
    ident = s[ident_start:i]
    if not ident.endswith("cfg"): return (False, start)

    i = _skip_ws_comments(s, i)
    if i >= len(s) or s[i] != "(": return (False, start)

    paren = 1; i += 1; inside = []
    while i < len(s) and paren > 0:
        if s[i] == "(": paren += 1
        elif s[i] == ")":
            paren -= 1
            if paren == 0:
                i += 1; break
        inside.append(s[i]); i += 1

    if paren != 0: return (False, start)
    i = _skip_ws_comments(s, i)
    if i >= len(s) or s[i] != "]": return (False, start)
    end_bracket = i

    contents = "".join(inside)
    if "test" in contents:
        return (True, end_bracket)
    return (False, start)

def _strip_cfg_test_item(code: str) -> Tuple[str, int]:
    i = 0; n = len(code)
    while i < n:
        i = _skip_ws_comments(code, i)
        if i >= n: break
        is_cfg, end_attr = _looks_like_cfg_test_attr(code, i)
        if not is_cfg:
            i += 1; continue

        attr_start = i
        j = end_attr + 1
        j = _skip_ws_comments(code, j)

        k = j; found_lbrace = -1
        while k < n:
            k = _skip_ws_comments(code, k)
            if k >= n: break
            if code[k] == "{":
                found_lbrace = k; break
            if code[k] == ";":
                return (_strip_block(code, attr_start, k), 1)
            if code[k].isalpha() or code[k] == "_":
                while k < n and (code[k].isalnum() or code[k] == "_"):
                    k += 1
            else:
                k += 1
        if found_lbrace != -1:
            end = _find_matching_brace(code, found_lbrace)
            if end == -1:
                return (code[:attr_start] + code[end_attr+1:], 1)
            return (_strip_block(code, attr_start, end), 1)
        return (code[:attr_start] + code[end_attr+1:], 1)
    return (code, 0)

def _strip_test_fn(code: str) -> Tuple[str, int]:
    i = 0; n = len(code)
    while i < n:
        i = _skip_ws_comments(code, i)
        if i >= n: break
        if not code.startswith("#[", i):
            i += 1; continue

        group_start = i
        saw_test_attr = False
        j = i
        while j < n and code.startswith("#[", j):
            t = j + 2; d = 1
            while t < n and d > 0:
                if code[t] == "[": d += 1
                elif code[t] == "]": d -= 1
                t += 1
            attr_body = code[j:t]
            if "test" in attr_body and attr_body.strip().startswith("#[test"):
                saw_test_attr = True
            j = _skip_ws_comments(code, t)

        if not saw_test_attr:
            i = j; continue

        k = j
        while True:
            k = _skip_ws_comments(code, k)
            if code.startswith("fn", k): break
            if code.startswith("#[", k):
                t = k + 2; d = 1
                while t < n and d > 0:
                    if code[t] == "[": d += 1
                    elif code[t] == "]": d -= 1
                    t += 1
                k = t; continue
            return (code[:group_start] + code[j:], 1)

        brace = code.find("{", k)
        if brace == -1:
            semi = code.find(";", k)
            end = semi if semi != -1 else j
            return (_strip_block(code, group_start, end), 1)
        end = _find_matching_brace(code, brace)
        if end == -1: end = brace
        return (_strip_block(code, group_start, end), 1)
    return (code, 0)

def _strip_mod_tests(code: str) -> Tuple[str, int]:
    i = 0; n = len(code)
    while i < n:
        i = _skip_ws_comments(code, i)
        if i >= n: break
        if code.startswith("mod", i):
            j = i + 3
            j = _skip_ws_comments(code, j)
            ident_start = j
            while j < n and (code[j].isalnum() or code[j] == "_"):
                j += 1
            ident = code[ident_start:j]
            if ident != "tests":
                i += 1; continue
            j = _skip_ws_comments(code, j)
            if j < n and code[j] == "{":
                end = _find_matching_brace(code, j)
                if end != -1:
                    return (_strip_block(code, i, end), 1)
        i += 1
    return (code, 0)

def strip_tests(code: str) -> Tuple[str, Dict[str, int]]:
    total_cfg = total_fn = total_mod = 0
    changed = True
    while changed:
        changed = False
        code2, n1 = _strip_cfg_test_item(code)
        if n1:
            total_cfg += n1; code = code2; changed = True; continue
        code2, n2 = _strip_test_fn(code)
        if n2:
            total_fn += n2; code = code2; changed = True; continue
        code2, n3 = _strip_mod_tests(code)
        if n3:
            total_mod += n3; code = code2; changed = True; continue
    return code, {
        "cfg_items_removed": total_cfg,
        "test_fns_removed": total_fn,
        "mod_tests_removed": total_mod
    }

def process_file(path: Path) -> Tuple[str, Dict[str, int]]:
    try:
        original = read_text(path)
    except Exception as e:
        print(f"[error]  {path}  read failed: {e}")
        return "ERROR", {"cfg_items_removed":0, "test_fns_removed":0, "mod_tests_removed":0}

    before_bytes = len(original.encode("utf-8", errors="ignore"))
    before_lines = original.count("\n") + (0 if original.endswith("\n") else 1)

    cleaned, stats = strip_tests(original)

    after_bytes = len(cleaned.encode("utf-8", errors="ignore"))
    after_lines = cleaned.count("\n") + (0 if cleaned.endswith("\n") else 1)

    try:
        if cleaned != original:
            write_text(path, cleaned)
            status = "CLEANED"
        else:
            status = "UNCHANGED"
    except Exception as e:
        print(f"[error]  {path}  write failed: {e}")
        return "ERROR", stats

    delta_b = before_bytes - after_bytes
    delta_l = before_lines - after_lines
    print(
        f"[{status:<9}] {path}  "
        f"bytes: {before_bytes}->{after_bytes} (-{delta_b}), "
        f"lines: {before_lines}->{after_lines} (-{delta_l}); "
        f"removed: cfg={stats['cfg_items_removed']}, "
        f"#[test]={stats['test_fns_removed']}, mod tests={stats['mod_tests_removed']}"
    )
    return status, stats

def main() -> None:
    if not ROOT.exists():
        print(f"error: root not found: {ROOT}", file=sys.stderr)
        sys.exit(1)

    print(f"[info] scanning root: {ROOT}")

    total_files = 0
    cleaned_files = 0
    unchanged_files = 0
    error_files = 0
    sum_cfg = sum_fn = sum_mod = 0

    for dirpath, _d, files in os.walk(ROOT):
        for name in files:
            if name in TARGET_FILENAMES:
                total_files += 1
                status, stats = process_file(Path(dirpath) / name)
                sum_cfg += stats.get("cfg_items_removed", 0)
                sum_fn  += stats.get("test_fns_removed", 0)
                sum_mod += stats.get("mod_tests_removed", 0)
                if status == "CLEANED": cleaned_files += 1
                elif status == "UNCHANGED": unchanged_files += 1
                else: error_files += 1

    print("\n[summary]")
    print(f"  files scanned (fixed.rs + vulnerable.rs): {total_files}")
    print(f"  cleaned:   {cleaned_files}")
    print(f"  unchanged: {unchanged_files}")
    print(f"  errors:    {error_files}")
    print(f"  total removed -> cfg items: {sum_cfg}, #[test] fns: {sum_fn}, mod tests: {sum_mod}")

    exit_code = 0 if error_files == 0 else 2
    print(f"\n[exit] code: {exit_code}")
    sys.exit(exit_code)

if __name__ == "__main__":
    main()
