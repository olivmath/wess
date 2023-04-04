def before_feature(context, feature):
    print(f"🏁 Start settings for {feature}...")
    context.wasm_path = "./wasm/here/BYTES_RESULT.txt"
    context.url = "http://127.0.0.1:7770"
    print("✅ Done for test!")

def after_feature(context, feature):
    ...