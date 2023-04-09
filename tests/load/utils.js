const URL = "http://127.0.0.1:7770"
const WASM = {
    func: "sum",
    return_type: "u8",
    args: [{
        type: "",
        name: ""
    }],
    wasm: [121]
}

function urlId(id) {
    return `${URL}/${id}`;
}

function checkWasm(obj) {
    const expected = Object.assign({}, obj.message.Success.metadata, {
        wasm: obj.message.Success.wasm
    });
    return compareObjects(expected, WASM);
}

function compareObjects(obj1, obj2) {
    function sorted(obj) {
        if (obj === null || typeof obj !== 'object') {
            return obj;
        }
        if (Array.isArray(obj)) {
            return obj.map(sorted);
        }
        return Object.keys(obj)
            .sort()
            .reduce((result, key) => {
                result[key] = sorted(obj[key]);
                return result;
            }, {});
    }

    return JSON.stringify(sorted(obj1)) === JSON.stringify(sorted(obj2));
}



export default {
    URL,
    WASM,
    urlId,
    checkWasm,
}