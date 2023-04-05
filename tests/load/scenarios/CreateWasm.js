import { post, get } from "k6/http";
import { sleep, check } from "k6";
import utils from "./utils.js";


export default function () {
    // Create a new Wasm module
    const response = post(utils.URL, JSON.stringify(utils.WASM));
    const id = JSON.parse(response.body).message;
    check(response, {
        'statuscode 200 - createWasm': (r) => r.status === 200,
        'max duration - createWasm': (r) => r.timings.duration < 4000
    })
    sleep(10);

    // Check if has created correctly
    const result = get(utils.urlId(id))
    const value = utils.checkWasm(JSON.parse(result.body))
    check(result, {
        'statuscode 200 - readWasm': (r) => r.status === 200,
        'max duration - readWasm': (r) => r.timings.duration < 4000,
    })
    check(value, {
        'check if has created correctly': (value) => value === true
    })
    sleep(1)
}