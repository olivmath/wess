from requests import Response, delete, post, put
from behave import when
from json import dumps
import uuid


@when('sending the wasm "{module_name}" to create a new WasmModule')
def create_module(context, module_name: str):
    config = {
        "url": context.url,
        "data": dumps(context.wasm_modules[module_name]),
        "timeout": 5000,
    }
    context.response: Response = post(**config)


@when('sending the wasm "{module_name}" to update the ID "{alias}"')
def update_module(context, module_name: str, alias: str):
    config = {
        "url": f"{context.url}/{context.ids[alias]}",
        "data": dumps(context.wasm_modules[module_name]),
        "timeout": 5000,
    }
    context.response: Response = put(**config)


@when('sending the ID "{alias}" to delete')
def delete_module(context, alias: str):
    config = {
        "url": f"{context.url}/{context.ids[alias]}",
        "timeout": 5000,
    }
    context.response: Response = delete(**config)


@when('the response status code is "{status_code}"')
def check_status_code(context, status_code: str):
    excepted = int(status_code)
    result = context.response.status_code

    msg = f"The response status code should be {excepted}, but was {result}"
    assert excepted == result, msg


@when("the response body matches the default UUID")
def check_module_id(context):
    module_id = context.response.json()["message"]["id"]
    try:
        uuid.UUID(module_id, version=4)
    except ValueError:
        assert False, f"Response '{module_id}' does not match the UUID pattern"


@when('the ID is saved in "{alias}"')
def save_module_id(context, alias: str):
    msg = f"Response does not contain the 'id' key\n{context.response.json()}"
    assert "id" in context.response.json()["message"], msg
    context.ids[alias] = context.response.json()["message"]["id"]
