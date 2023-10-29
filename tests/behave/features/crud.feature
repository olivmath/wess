Feature: Test WasmModule CRUD Operations

    Background:
        Given the following WasmModules are available:
            | module     | functions                                                 | returns                   | args                      |
            | the_answer | [the_answer]                                              | [I32]                     | []                        |
            | fibonacci  | [fibonacci]                                               | [I32]                     | [I64]                     |
            | sum/sumi32 | [sum]                                                     | [I32]                     | [I32, I32]                |
            | sum/sumf32 | [sum]                                                     | [F32]                     | [F32, F32]                |
            | sum/sumi64 | [sum]                                                     | [I64]                     | [I64, I64]                |
            | sum/sumf64 | [sum]                                                     | [F64]                     | [F64, F64]                |
            | syscall    | [random, random_numbers, random_numbers_len, free_memory] | [[I32], [I32], [I32], []] | [[I32], [I32], [], [I32]] |

    Scenario: Create a WasmModule
        When sending the wasm "the_answer" to create a new WasmModule
        And the response status code is "202"
        And the response body matches the default UUID
        And the ID is saved in "the_answer_created"
        Then Wess must log the "CREATE" operation with the ID "the_answer_created"
        And log must matches the pattern "(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}) \| (ERROR|WARN|INFO|DEBUG|TRACE) (wess|wess::tx|wess::err) \| (src\/\S+\.rs:\d+) - (.+)"

    Scenario: Updating a WasmModule
        When sending the wasm "fibonacci" to update the ID "the_answer_created"
        And the response status code is "202"
        Then Wess must log the "UPDATE" operation with the ID "the_answer_created"
        And log must matches the pattern "(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}) \| (ERROR|WARN|INFO|DEBUG|TRACE) (wess|wess::tx|wess::err) \| (src\/\S+\.rs:\d+) - (.+)"

    Scenario: Delete a WasmModule
        When sending the ID "the_answer_created" to delete
        And the response status code is "202"
        Then Wess must log the "DELETE" operation with the ID "the_answer_created"
        And log must matches the pattern "(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}) \| (ERROR|WARN|INFO|DEBUG|TRACE) (wess|wess::tx|wess::err) \| (src\/\S+\.rs:\d+) - (.+)"
