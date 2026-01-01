#!/usr/bin/env python3
from transformers import AutoTokenizer, AutoModelForCausalLM
import torch

MODEL_ID = "deepseek-ai/deepseek-coder-6.7b-instruct"

# Load tokenizer & model
tokenizer = AutoTokenizer.from_pretrained(MODEL_ID, trust_remote_code=True)
device = "cuda" if torch.cuda.is_available() else "cpu"
dtype = torch.bfloat16 if device == "cuda" else torch.float32
model = AutoModelForCausalLM.from_pretrained(MODEL_ID, trust_remote_code=True, torch_dtype=dtype).to(device)

# Prompt (kept exactly as requested)
messages = [
    {"role": "user", "content": "Which is the biggest country by population"}
]

# Build prompt with the chat template, then tokenize
prompt = tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
inputs = tokenizer(prompt, return_tensors="pt").to(device)

# Generate
outputs = model.generate(
    **inputs,
    max_new_tokens=64,
    do_sample=False,
    eos_token_id=tokenizer.eos_token_id,
)

# Print ONLY the assistant's response (strip the prompt part)
new_tokens = outputs[0, inputs["input_ids"].shape[-1]:]
print(tokenizer.decode(new_tokens, skip_special_tokens=True).strip())
