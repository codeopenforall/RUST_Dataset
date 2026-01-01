import json
from pathlib import Path
import argparse

def collect_rs_files(dir_path: Path, label: str):
    """Yield dataset records for all .rs files under dir_path with the given label."""
    for p in sorted(dir_path.rglob("*.rs")):
        try:
            code = p.read_text(encoding="utf-8", errors="ignore")
        except Exception as e:
            print(f"[!] Skipping {p} (read error: {e})")
            continue

        yield {
            "instruction": "You are a strict Rust security reviewer. Classify the snippet. Output EXACTLY ONE token chosen from this set: {fixed, vulnerable}. Do not add punctuation, spaces, quotes, or explanations.",
            "input": code,
            "output": label
        }

def main():
    parser = argparse.ArgumentParser(description="Build dataset from Positive/Negative Rust files.")
    parser.add_argument(
        "--root",
        default="/home/ikqnm/Downloads/Cleaned_RUST",
        help="Root directory containing Positive/ and Negative/ subfolders."
    )
    parser.add_argument(
        "--out",
        default= "Cleaned_Data_For_Evaluation", #"real_Dataset.json",
        help="Output JSON file path."
    )
    # Optional: choose the fixed label exactly (e.g., 'fixed' or 'fixed.rs')
    parser.add_argument(
        "--fixed-label",
        default="fixed",
        help="Label to use for Negative examples ('fixed' or 'fixed.rs')."
    )
    args = parser.parse_args()

    root = Path(args.root).expanduser().resolve()
    pos_dir = root / "Positive"
    neg_dir = root / "Negative"

    if not pos_dir.exists():
        raise FileNotFoundError(f"Missing directory: {pos_dir}")
    if not neg_dir.exists():
        raise FileNotFoundError(f"Missing directory: {neg_dir}")

    records = []

    # Positive => vulnerable
    records.extend(collect_rs_files(pos_dir, label="vulnerable"))
    # Negative => fixed (or fixed.rs if you pass --fixed-label fixed.rs)
    records.extend(collect_rs_files(neg_dir, label="fixed"))

    # Optional: stable sort by label then by length to keep output consistent across runs
    records.sort(key=lambda r: (r["output"], len(r["instruction"])))

    out_path = Path(args.out).resolve()
    out_path.write_text(json.dumps(list(records), ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"[+] Wrote {len(records)} records to {out_path}")

if __name__ == "__main__":
    main()
