from requests import post, get, put, delete
from json import dumps, loads
from behave import given, when, then

def text2bytes(text):
    return [ int(i) for i in text.split(",")]



@given('a WebAssembly module called "{module_name}" with function "{function_name}", with "{return_type}", with this args')
def get_wasm(context, module_name, function_name, return_type):
    wasm_file = context.wasm_path.replace("here", module_name)
    with open(wasm_file) as wasm_module:
        context.payload = {
            "wasm": text2bytes(wasm_module.read()),
            "func": function_name,
            "return_type": return_type,
            "args": loads(context.text)
        }


@when('I create the module')
def send_post(context):
   context.response = post(context.url, data=dumps(context.payload))


@then('the response status code should be "{status}"')
def check_status_code(context, status):
    assert context.response.status_code == int(status), context.response


@when('update module "{id}"')
def send_put(context, id):
    context.response = put(context.url + f"/{id}", data=dumps(context.payload))


@when('I remove module "{id}"')
def send_delete(context, id):
    context.response = delete(context.url + f"/{id}")


@when('run module "{id}" with args')
def send_run(context, id):
    args = loads(context.text)
    context.response = post(context.url + f"/{id}", data=dumps(args))



@then('should response with "{response}"')
def step_impl(context, response):
    r = context.response.json()
    assert int(r['message']['Success']) == int(response), r