#!/usr/bin/env python3
"""
Recursively rename every 'oracle.rs' to 'demo_test.rs' under the given root.

Default root:
  /home/ikqnm/PycharmProjects/PythonProject/O3_mini/DataGenerated_pairs_new/

Usage:
  python rename_oracle_to_demo_test.py
  python rename_oracle_to_demo_test.py --root /path/to/root
  python rename_oracle_to_demo_test.py --force
"""

from pathlib import Path
import argparse
import sys

DEFAULT_ROOT = "/home/ikqnm/PycharmProjects/PythonProject/O3_mini/Cleaned_DataGenerated_pairs_new/"

def rename_all(root: Path, force: bool = False) -> None:
    if not root.exists():
        print(f"ERROR: Root not found: {root}", file=sys.stderr)
        sys.exit(1)

    renamed = 0
    skipped = 0
    errors  = 0

    for src in root.rglob("oracle.rs"):
        dst = src.with_name("demo_test.rs")

        try:
            if dst.exists() and not force:
                print(f"[skip] {src} -> {dst} (target exists; use --force to overwrite)")
                skipped += 1
                continue

            if dst.exists() and force:
                dst.unlink()

            src.rename(dst)
            print(f"[ok]   {src} -> {dst}")
            renamed += 1

        except Exception as e:
            print(f"[err]  {src}: {e}", file=sys.stderr)
            errors += 1

    print("\n[summary]")
    print(f"  renamed : {renamed}")
    print(f"  skipped : {skipped}")
    print(f"  errors  : {errors}")

def main():
    parser = argparse.ArgumentParser(description="Rename all oracle.rs files to demo_test.rs recursively.")
    parser.add_argument("--root", default=DEFAULT_ROOT, help="Root directory to search")
    parser.add_argument("--force", action="store_true", help="Overwrite existing demo_test.rs if present")
    args = parser.parse_args()

    root = Path(args.root).expanduser().resolve()
    rename_all(root, force=args.force)

if __name__ == "__main__":
    main()
