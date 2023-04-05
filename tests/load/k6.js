import CreateWasm from "./scenarios/CreateWasm.js"
import { group, sleep } from 'k6'
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";

export function handleSummary(data) {
    return {
        "summary.html": htmlReport(data),
    };
}
export let options = {
    vus: 50000,
    duration: "30s",
};

export default function () {
    group("Create and Read Wasm", () => {
        CreateWasm()
    })

    sleep(1)
}
