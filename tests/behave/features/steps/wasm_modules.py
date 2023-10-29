import re
from behave import given


def text2bytes(text):
    return [int(i) for i in text.split(",")]


def str_to_list(s):
    return re.findall(r"\w+", s)


@given("the following WasmModules are available")
def define_wasm_modules(context):
    wasm_modules = {}
    for row in context.table:
        name = row["module"]
        functions = row["functions"]
        returns = row["returns"]
        args = row["args"]

        wasm_file = context.wasm_path.replace("here", name)

        with open(wasm_file) as wasm_module:
            wasm_modules[name] = {
                "wasm": text2bytes(wasm_module.read()),
                "metadata": {
                    "functionName": str_to_list(functions)[0],
                    "returnType": str_to_list(returns),
                    "args": str_to_list(args),
                },
            }
    context.wasm_modules = wasm_modules
