export function uint8ArrayToBase64(arr) {
    return Buffer.from(arr).toString("base64");
}

export function base64ToUint8Array(base64String) {
    const buffer = Buffer.from(base64String, "base64");
    const uint8Array = new Uint8Array(buffer);
    return uint8Array;
}

export function createFheHeader(fheMethod, fheVersion) {
    const header = {
        "fhe-method": fheMethod,
        "fhe-version": fheVersion,
        "zk-experiment": "true",
    };
    return header;
}
