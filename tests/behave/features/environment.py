import os
import subprocess
import time
import toml


def modify_wess_toml(stage, url, port):
    wess_toml_path = "wess.toml"
    with open(wess_toml_path, "r") as toml_file:
        wess_toml = toml.load(toml_file)
        wess_toml["database"]["db"] = stage
        wess_toml["server"]["address"] = url
        wess_toml["server"]["port"] = port
    with open(wess_toml_path, "w") as toml_file:
        toml.dump(wess_toml, toml_file)


def before_all(context):
    print("🏁 Start settings...")
    context.wasm_path = "./wasm/here/BYTES_RESULT.txt"

    print("📡 Setting Wess: stage = dev, url = http://127.0.0.1:7770")
    modify_wess_toml("dev", "127.0.0.1", 7770)
    context.url = "http://127.0.0.1:7770"

    print("🚀 Run Wess: stage = dev")
    context.process = subprocess.Popen(["cargo", "run"])
    time.sleep(3.0)

    print("📄 Get log path: log/wess.toml")
    context.log_path = "log/wess.log"

    context.wasm_modules = {}
    context.ids = {}
    print("✅ Done for test!")


def after_all(context):
    context.process.terminate()
    context.process.wait()
    modify_wess_toml("prod", "0.0.0.0", 80)

    os.remove("log/wess.log")
    os.system("rm -rf rocksdb")
