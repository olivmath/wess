import { post, get } from "k6/http";
import { sleep, check } from "k6";
import { Rate } from "k6/metrics";
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";
import sumJson from "./payloads/sum_f32_f32.js"
import fibJson from "./payloads/fibonacci.js"
const baseUrl = "http://127.0.0.1:7770"



export const SuccessRunFunction = new Rate('SuccessRunFunction');


export function handleSummary(data) {
    return {
        "summary.html": htmlReport(data),
    };
}


export let options = {
    scenarios: {
        // smoke: {
        //     executor: "constant-vus",
        //     vus: 10,
        //     duration: '5s',
        // },
        // load: {
        //     startTime: '5s',
        //     executor: 'ramping-vus',
        //     startVUs: 0,
        //     stages: [
        //         { duration: '5s', target: 100 },
        //         { duration: '10s', target: 100 },
        //         { duration: '5s', target: 0 },
        //     ],
        // },
        stress: {
            startTime: '0s',
            executor: "ramping-arrival-rate",
            preAllocatedVUs: 5000,
            timeUnit: "1s",
            stages: [
                { duration: '10s', target: 100 },
                { duration: '30s', target: 100 },
                { duration: '10s', target: 500 },
                { duration: '30s', target: 500 },
                { duration: '5s', target: 1000 },
                { duration: '30s', target: 1000 },
                { duration: '10s', target: 0 },
            ],
        },
        // peak: {
        //     startTime: '0s',
        //     executor: "ramping-arrival-rate",
        //     preAllocatedVUs: 10000,
        //     timeUnit: "1s",
        //     stages: [
        //         { duration: '5s', target: 100 },
        //         { duration: '10s', target: 100 },
        //         { duration: '5s', target: 10000 },
        //         { duration: '10s', target: 10000 },
        //         { duration: '5s', target: 100 },
        //         { duration: '10s', target: 100 },
        //         { duration: '5s', target: 0 },
        //     ],
        // }
    },
    thresholds: {
        'SuccessRunFunction': ['rate>0.9'],
    }
}


let COUNTER = 0

export default function () {
    // Save a new Wasm module
    const createResponse = sum.create()
    check(createResponse, {
        'status 200 - write': (r) => r.status === 200,
        'max duration - write': (r) => r.timings.duration < 4000
    })
    COUNTER += 1
    sleep(1);
}

export function teardown(data) {
    const r = get(baseUrl)
    console.log(r.json().message)
    console.log(COUNTER)
    console.log(COUNTER == r.json().message)
    SuccessRunFunction.add(COUNTER == r.json().message)
}

// export default function () {

//     const x = parseInt(Math.random() * 100000)
//     const y = parseInt(Math.random() * 100000)
//     const runResponse = sum.run(x, y, "4d0dc0c93b9c5a49670b1c27f8d68db5b54aa2f5f09741e5d3f26117ed58bbd5")

//     check(runResponse, {
//         'status 200 - run': (r) => r.status === 200,
//         'max duration - run': (r) => r.timings.duration < 4000,
//     })

//     SuccessRunFunction.add(x + y === JSON.parse(runResponse.body).message[0])
//     sleep(1)
// }


const sum = {
    create: () => {
        return post(baseUrl, JSON.stringify(sumJson));
    },
    run: (x, y, id) => {
        return post(`${baseUrl}/${id}`, JSON.stringify([x, y]));
    }
}

const fibonacci = {
    create: () => {
        return post(baseUrl, fibJson);
    },
    run: (x, y, id) => {
        return post(`${baseUrl}/${id}`, JSON.stringify([x, y]));
    }
}

function getWasm(id) {
    return get(`${baseUrl}/${id}`)
}