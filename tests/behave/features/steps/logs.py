from behave import then
import re


@then('Wess must log the "{operation}" operation with the ID "{alias}"')
def check_ops_in_logs(context, operation: str, alias: str):
    module_id = context.ids[alias]

    with open(context.log_path, "r") as log_file:
        log_lines = log_file.readlines()

    found = any(re.search(rf"{operation} {module_id}", line) for line in log_lines)

    msg = f"No log entry for '{operation}' operation with ID '{module_id}' found."
    assert found, msg


@then('log must matches the pattern "{pattern}"')
def check_logs_pattern(context, pattern: str):
    with open(context.log_path, "r") as log_file:
        lines = log_file.readlines()
        non_matching_lines = [line for line in lines if not re.search(pattern, line)]

    if non_matching_lines:
        print("The following log lines do not match the pattern:")
        for line in non_matching_lines:
            print(line)

    assert not non_matching_lines, "Some log lines don't match the pattern."
