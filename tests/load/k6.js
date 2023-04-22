import { post, get } from "k6/http";
import { sleep, check } from "k6";
import { Rate } from "k6/metrics";
import utils from "./utils.js";
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";


export const SavedCorrectly = new Rate('SavedCorrectly');


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
            duration: '10s',
        },
        load: {
            startTime: '15s',
            executor: 'ramping-vus',
            startVUs: 0,
            stages: [
                { duration: '5s', target: 100 },
                { duration: '10s', target: 100 },
                { duration: '5s', target: 0 },
            ],
        },
        stress: {
            startTime: '45s',
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
            startTime: '95s',
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
    const response = post(utils.URL, JSON.stringify(utils.WASM));
    const id = JSON.parse(response.body).message;
    check(response, {
        'status 200 - write': (r) => r.status === 200,
        'max duration - write': (r) => r.timings.duration < 4000
    })
    sleep(1);

    // Retrieve the Wasm module
    const result = get(utils.urlId(id))
    check(result, {
        'status 200 - read': (r) => r.status === 200,
        'max duration - read': (r) => r.timings.duration < 4000,
    })

    // Check if Wasm module has saved correctly
    SavedCorrectly.add(utils.checkWasm(JSON.parse(result.body)))
    sleep(1)
}
