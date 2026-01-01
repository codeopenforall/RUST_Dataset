import json
from pathlib import Path

def build_dataset(
    root_dir: str = "/home/ikqnm/PycharmProjects/PythonProject/Data Augmentation/Qwen2.5-Coder/DataAugmented_pairs_new_A3",
    output_file: str = "/home/ikqnm/PycharmProjects/PythonProject/Data Augmentation/Qwen2.5-Coder/Augmented_dataset_from_pairs_A3.json"
):
    root = Path(root_dir)
    if not root.exists():
        raise FileNotFoundError(f"Input directory not found: {root.resolve()}")

    records = []

    # Recursively find vulnerable.rs and fixed.rs
    for rs_file in root.rglob("*.rs"):
        if rs_file.name not in ("vulnerable.rs", "fixed.rs", "A1_fixed.rs", "A2_fixed.rs", "A3_fixed.rs", "A1_vulnerable.rs", "A2_vulnerable.rs", "A3_vulnerable.rs"):
            continue

        try:
            code = rs_file.read_text(encoding="utf-8", errors="ignore")
        except Exception as e:
            print(f"[!] Skipping {rs_file} due to read error: {e}")
            continue

        # Map filename -> output label (as you specified)
        output_label = "vulnerable" if "vulnerable" in rs_file.name.lower() else "fixed"

        records.append({
            "instruction": "Classify the Rust snippet as exactly one word: vulnerable or fixed.",
            "input": code,
            "output": output_label
        })

    # (Optional) sort for stable output
    # Put vulnerable entries before fixed in each directory, then by path
    records.sort(key=lambda r: (r["output"] != "vulnerable",))  # simple stable sort

    # Write JSON array
    Path(output_file).write_text(
        json.dumps(records, ensure_ascii=False, indent=2),
        encoding="utf-8"
    )
    print(f"[+] Wrote {len(records)} records to {output_file}")

if __name__ == "__main__":
    build_dataset()
