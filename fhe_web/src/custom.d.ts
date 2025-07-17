interface FheHeader {
    "fhe-method": string;
    "fhe-version": string;
    "zk-experimental": string;
}

export function uint8ArrayToBase64(arr: Uint8Array): string;
export function base64ToUint8Array(base64: String): Uint8Array;
export function createFheHeader(fheMethod: String, fheVersion: String): FheHeader;
