import json
from sklearn.metrics import confusion_matrix, classification_report

# Path to your predictions file
pred_file = "/home/ikqnm/PycharmProjects/PythonProject/LLaMA-Factory/Output/EVL/evl_Llama3_2_A1A2A3O3/generated_predictions.cleaned.jsonl"

def normalize(label: str):
    """Convert text labels to numeric values: fixed=0, vulnerable=1."""
    if not label:
        return -1
    s = label.strip().lower()
    if s.__contains__("fixed"):
        return 0
    elif s.__contains__("vulnerable"):
        return 1
    else:
        return -1

# Collect predictions and gold labels
preds, golds = [], []
with open(pred_file, "r", encoding="utf-8") as f:
    for line in f:
        obj = json.loads(line)
        p = normalize(obj.get("predict", ""))
        g = normalize(obj.get("label", ""))
        if p != -1 and g != -1:  # skip invalids
            preds.append(p)
            golds.append(g)

# Compute confusion matrix
cm = confusion_matrix(golds, preds, labels=[0, 1])
print("Confusion Matrix:")
print(cm)

# Detailed classification report
report = classification_report(
    golds, preds,
    target_names=["fixed (0)", "vulnerable (1)"],
    zero_division=0
)
print("\nClassification Report:")
print(report)
