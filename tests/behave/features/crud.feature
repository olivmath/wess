Feature: Testing the Wess API

    Scenario: Creating a new module
        Given a WebAssembly module called "the_answer" with function "the_answer", with "i32", with this args
            """
            [
                {
                    "type": "",
                    "name": ""
                }
            ]
            """
        When I create the module
        Then the response status code should be "200"

    # Scenario: Updating an existing module
    #     Given a WebAssembly module called "the_answer" with function "answer", with "i32", with this args
    #         """
    #         [
    #             {
    #                 "type": "",
    #                 "name": ""
    #             }
    #         ]
    #         """
    #     When update module "7e3a7557831502ce9b600225e21e29b38240b460eb7bbaade1eb6e73af03e7ec"
    #     Then the response status code should be "200"

    # Scenario: Deleting an existing module
    #     When I remove module "7e3a7557831502ce9b600225e21e29b38240b460eb7bbaade1eb6e73af03e7ec"
    #     Then the response status code should be "200"

    Scenario: Running a function in an existing module
        When run module "0187280178985429a5d57aa7e1d40d48d6da3c5881488cd5cdbe190b07cbaa75" with args
            """
            {
                "args": []
            }
            """
        Then the response status code should be "200"
        And should response with "42"
