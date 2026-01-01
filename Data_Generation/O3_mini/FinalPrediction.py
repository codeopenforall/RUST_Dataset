import json, re, sys

in_path  = "/home/ikqnm/PycharmProjects/PythonProject/LLaMA-Factory/Output/EVL/evl_Llama3_2_A1A2A3O3/generated_predictions.jsonl"
out_path = "/home/ikqnm/PycharmProjects/PythonProject/LLaMA-Factory/Output/EVL/evl_Llama3_2_A1A2A3O3/generated_predictions.cleaned.jsonl"

def to_label(p: str, l: str) -> str:
    if not p:
        if l.__contains__("fixed"):
            return "EMPTY_fixed"
        else:
            return "EMPTY_vulnerable"

    # keep only a-z letters from the first word
    m = re.search(r"[A-Za-z]+", p)
    w = (m.group(0).lower() if m else "")

    if "fix" in w:
        return "fixed"
    elif "vul" in w:
        return "vulnerable"
    else:
        if l.__contains__("fixed"):
            return "EMPTY_fixed"
        else:
            return "EMPTY_vulnerable"
        #return "vulnerable"  # fallback

with open(in_path, "r", encoding="utf-8") as fin, open(out_path, "w", encoding="utf-8") as fout:
    for line in fin:
        obj = json.loads(line)
        obj["predict"] = to_label(obj.get("predict", ""), obj.get("label", ""))
        fout.write(json.dumps(obj, ensure_ascii=False) + "\n")

print(f"Wrote cleaned predictions to {out_path}")
