#!/usr/bin/env python3
"""
Strip comments from Rust sources (vulnerable.rs, fixed.rs, oracle.rs) while preserving strings/char literals,
including nested block comments, raw strings (r###"..."###, br###"..."###), and escapes.
After stripping, delete any line that has no code left (only whitespace).

Input:  /home/ikqnm/PycharmProjects/PythonProject/DataGenerated_pairs_new
Output: /home/ikqnm/PycharmProjects/PythonProject/Cleaned_DataGenerated_pairs_new
"""

from pathlib import Path
import argparse

def strip_rust_comments(src: str) -> str:
    NORMAL, LINE_COMMENT, BLOCK_COMMENT, STRING, RAW_STRING, CHAR = range(6)

    out = []
    i = 0
    n = len(src)
    state = NORMAL
    block_depth = 0
    raw_hashes = 0

    while i < n:
        ch = src[i]

        if state == NORMAL:
            if ch == '/':
                # Lookahead
                nxt = src[i+1] if i+1 < n else ''
                if nxt == '/':
                    # line comment: // or /// or //! -> treat same
                    state = LINE_COMMENT
                    i += 2
                    continue
                elif nxt == '*':
                    # block comment: /* ... */ with nesting
                    state = BLOCK_COMMENT
                    block_depth = 1
                    i += 2
                    continue
                else:
                    out.append(ch)
                    i += 1
                    continue

            # Strings (normal and byte strings)
            if ch in ('b', 'B'):
                # possible b"..." or br###"..."###
                j = i + 1
                if j < n and src[j] == 'r':
                    # br raw string
                    k = j + 1
                    hashes = 0
                    while k < n and src[k] == '#':
                        hashes += 1
                        k += 1
                    if k < n and src[k] == '"':
                        state = RAW_STRING
                        raw_hashes = hashes
                        i = k + 1
                        continue
                elif j < n and src[j] == '"':
                    # b"..."
                    state = STRING
                    out.append(src[i])      # 'b'
                    out.append(src[j])      # opening '"'
                    i = j + 1
                    continue
                # fallthrough: lone 'b'
                out.append(ch)
                i += 1
                continue

            if ch == 'r':
                # possible r###"..."###
                j = i + 1
                hashes = 0
                while j < n and src[j] == '#':
                    hashes += 1
                    j += 1
                if j < n and src[j] == '"':
                    state = RAW_STRING
                    raw_hashes = hashes
                    i = j + 1
                    continue
                # not a raw string
                out.append(ch)
                i += 1
                continue

            if ch == '"':
                state = STRING
                out.append(ch)
                i += 1
                continue

            if ch == "'":
                # char literal (may be lifetime `'a` too; we conservatively treat as char if it closes soon)
                # If pattern looks like lifetime (apostrophe + ident), just pass through.
                j = i + 1
                if j < n and (src[j].isalpha() or src[j] == '_'):
                    # Could be lifetime/label; emit as normal
                    out.append(ch)
                    i += 1
                    continue
                # else handle as char literal
                state = CHAR
                out.append(ch)
                i += 1
                continue

            out.append(ch)
            i += 1

        elif state == LINE_COMMENT:
            # consume until newline, but keep the newline
            if ch == '\n':
                out.append('\n')
                state = NORMAL
            i += 1

        elif state == BLOCK_COMMENT:
            # support nested /* ... */
            if ch == '/' and i + 1 < n and src[i+1] == '*':
                block_depth += 1
                i += 2
                continue
            if ch == '*' and i + 1 < n and src[i+1] == '/':
                block_depth -= 1
                i += 2
                if block_depth == 0:
                    state = NORMAL
                continue
            # otherwise, skip characters inside block comment (including newlines)
            i += 1

        elif state == STRING:
            # normal string, honor escapes
            if ch == '\\':
                # copy escape pair
                out.append(ch)
                if i + 1 < n:
                    out.append(src[i+1])
                    i += 2
                else:
                    i += 1
                continue
            out.append(ch)
            i += 1
            if ch == '"':
                state = NORMAL

        elif state == RAW_STRING:
            # close when we see a quote followed by raw_hashes of '#'
            if ch == '"':
                # check for closing hashes
                k = i + 1
                count = 0
                while count < raw_hashes and k < n and src[k] == '#':
                    count += 1
                    k += 1
                if count == raw_hashes:
                    # close raw string
                    out.append('"')
                    out.extend('#' * raw_hashes)
                    i = k
                    state = NORMAL
                    continue
                else:
                    # it's just a quote inside raw string
                    out.append('"')
                    i += 1
                    continue
            # normal char inside raw string
            out.append(ch)
            i += 1

        elif state == CHAR:
            # char literal, honor escapes until closing unescaped '
            if ch == '\\':
                out.append(ch)
                if i + 1 < n:
                    out.append(src[i+1])
                    i += 2
                else:
                    i += 1
                continue
            out.append(ch)
            i += 1
            if ch == "'":
                state = NORMAL

    # Join the stripped content
    stripped = "".join(out)

    # Post-process: drop lines that contain no code (only whitespace)
    nonempty_lines = [line for line in stripped.splitlines() if line.strip() != ""]
    return ("\n".join(nonempty_lines) + "\n") if nonempty_lines else ""

def process_tree(src_root: Path, dst_root: Path):
    # Only process vulnerable.rs, fixed.rs, oracle.rs; mirror directories as needed.
    for path in src_root.rglob("*"):
        if path.is_dir():
            # mirror directory
            rel = path.relative_to(src_root)
            (dst_root / rel).mkdir(parents=True, exist_ok=True)
            continue

        rel = path.relative_to(src_root)
        dst = dst_root / rel

        if path.name in ("vulnerable.rs", "fixed.rs", "oracle.rs"):
            try:
                code = path.read_text(encoding="utf-8", errors="ignore")
            except Exception as e:
                print(f"[!] Failed to read {path}: {e}")
                continue
            cleaned = strip_rust_comments(code)
            try:
                dst.write_text(cleaned, encoding="utf-8")
                print(f"[+] Cleaned -> {dst}")
            except Exception as e:
                print(f"[!] Failed to write {dst}: {e}")
        else:
            # Optional: copy other files untouched so the full structure stays intact.
            try:
                data = path.read_bytes()
                dst.write_bytes(data)
            except Exception as e:
                # Non-fatal; skip if you only care about .rs files
                print(f"[i] Skipped copy {path} ({e})")

def main():
    parser = argparse.ArgumentParser(description="Remove comments from Rust files and drop empty lines, preserving directory structure.")
    parser.add_argument("--input", default="/home/ikqnm/PycharmProjects/PythonProject/O3_mini/DataGenerated_pairs_new",
                        help="Input root directory")
    parser.add_argument("--output", default="/home/ikqnm/PycharmProjects/PythonProject/O3_mini/Cleaned_DataGenerated_pairs_new",
                        help="Output root directory (will be created)")
    args = parser.parse_args()

    src_root = Path(args.input).expanduser().resolve()
    dst_root = Path(args.output).expanduser().resolve()
    dst_root.mkdir(parents=True, exist_ok=True)

    process_tree(src_root, dst_root)
    print(f"[✓] Done. Output at: {dst_root}")

if __name__ == "__main__":
    main()
