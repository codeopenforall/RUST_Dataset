#!/usr/bin/env python3
"""
Strip comments from all Rust (.rs) files while preserving strings/char literals,
including nested block comments, raw strings (r###"..."###, br###"..."###), and escapes.
After stripping, any line that has no code (only whitespace) is removed.

Input:  /home/ikqnm/Downloads/RUST
Output: /home/ikqnm/Downloads/Cleaned_RUST
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
    string_is_byte = False  # for b"..." and br###"..."###
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
                        string_is_byte = True
                        i = k + 1
                        continue
                elif j < n and src[j] == '"':
                    # b"..."
                    state = STRING
                    string_is_byte = True
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
                    string_is_byte = False
                    i = j + 1
                    continue
                # not a raw string
                out.append(ch)
                i += 1
                continue

            if ch == '"':
                state = STRING
                string_is_byte = False
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
            # otherwise, skip characters inside block comment
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
                string_is_byte = False

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
                    string_is_byte = False
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
    # Keep consistent Unix newlines and ensure file ends with a trailing newline.
    nonempty_lines = [line for line in stripped.splitlines() if line.strip() != ""]
    return ("\n".join(nonempty_lines) + "\n") if nonempty_lines else ""

def process_tree(src_root: Path, dst_root: Path):
    # Clean every .rs file and mirror the directory structure into dst_root.
    for path in src_root.rglob("*.rs"):
        rel = path.relative_to(src_root)
        dst = dst_root / rel
        dst.parent.mkdir(parents=True, exist_ok=True)

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

def main():
    parser = argparse.ArgumentParser(
        description="Remove comments from all Rust files and drop empty lines (post-strip)."
    )
    parser.add_argument("--input", default="/home/ikqnm/Downloads/PrimaryData_syeda",
                        help="Input root directory")
    parser.add_argument("--output", default="/home/ikqnm/Downloads/Cleaned_PrimaryData_syeda",
                        help="Output root directory (will be created)")
    args = parser.parse_args()

    src_root = Path(args.input).expanduser().resolve()
    dst_root = Path(args.output).expanduser().resolve()
    dst_root.mkdir(parents=True, exist_ok=True)

    process_tree(src_root, dst_root)
    print(f"[✓] Done. Output at: {dst_root}")

if __name__ == "__main__":
    main()
