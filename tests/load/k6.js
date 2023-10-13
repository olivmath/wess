import { post, get } from "k6/http";
import { sleep, check } from "k6";
import { Rate } from "k6/metrics";
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";
import sum from "./payloads/sum_f32_f32.json"
import fib from "./payloads/fibonacci.json"

const baseUrl = "http://127.0.0.1:7770"
export const SuccessRunFunction = new Rate('SuccessRunFunction');


export function handleSummary(data) {
    return {
        "summary.html": htmlReport(data),
    };
}


export let options = {
    scenarios: {
        smoke: {
            executor: "constant-vus",
            vus: 10,
            duration: '5s',
        },
        load: {
            startTime: '5s',
            executor: 'ramping-vus',
            startVUs: 0,
            stages: [
                { duration: '5s', target: 100 },
                { duration: '10s', target: 100 },
                { duration: '5s', target: 0 },
            ],
        },
        stress: {
            startTime: '25s',
            executor: "ramping-arrival-rate",
            preAllocatedVUs: 5000,
            timeUnit: "1s",
            stages: [
                { duration: '5s', target: 100 },
                { duration: '10s', target: 100 },
                { duration: '5s', target: 500 },
                { duration: '10s', target: 500 },
                { duration: '5s', target: 1000 },
                { duration: '10s', target: 1000 },
                { duration: '5s', target: 0 },
            ],
        },
        peak: {
            startTime: '75s',
            executor: "ramping-arrival-rate",
            preAllocatedVUs: 10000,
            timeUnit: "1s",
            stages: [
                { duration: '5s', target: 100 },
                { duration: '10s', target: 100 },
                { duration: '5s', target: 10000 },
                { duration: '10s', target: 10000 },
                { duration: '5s', target: 100 },
                { duration: '10s', target: 100 },
                { duration: '5s', target: 0 },
            ],
        }
    },
    thresholds: {
        'SavedCorrectly': ['rate>0.9'],
    }
}



export default function () {
    // Save a new Wasm module
    const createResponse = sum.create()
    const id = JSON.parse(createResponse.body).message;
    check(createResponse, {
        'status 200 - write': (r) => r.status === 200,
        'max duration - write': (r) => r.timings.duration < 4000
    })
    sleep(1);

    // Retrieve the Wasm module
    const getResponse = getWasm(id)
    check(getResponse, {
        'status 200 - read': (r) => r.status === 200,
        'max duration - read': (r) => r.timings.duration < 4000,
    })
    
    // Run function the wasm module
    const x = parseInt(Math.random() * 1000000)
    const y = parseInt(Math.random() * 1000000)
    const runResponse = sum.run(x, y, id)
    check(runResponse, {
        'status 200 - read': (r) => r.status === 200,
        'max duration - read': (r) => r.timings.duration < 4000,
    })

    SuccessRunFunction.add(x + y === runResponse.body[0])
}

const sum = {
    create: () => {
        const payload = sum
        return post(baseUrl, JSON.stringify(payload));
    },
    run: (x, y, id) => {
        return post(`${baseUrl}/${id}`, JSON.stringify([x, y]));
    }
}

const fibonacci = {
    create: () => {

        const payload = fib
        return post(baseUrl, JSON.stringify(payload));
    },
    run: (x, y, id) => {
        return post(`${baseUrl}/${id}`, JSON.stringify([x, y]));
    }
}

function getWasm(id) {
    return get(`${baseUrl}/${id}`)
}