// This module is the CJS entry point for the library.

// The Rust addon.
import * as addon from "./load.cjs";

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.

type Bytes = Uint8Array;
declare module "./load.cjs" {
    // fhe
    function create_config(params?: Bytes): Bytes;
    function generate_client_key(config: Bytes): Bytes;
    function generate_server_key(client_key: Bytes): Bytes;
    function generate_public_key(client_key: Bytes): Bytes;
    function generate_public_zk_params(max_num_message: Number, params?: Bytes): Bytes;

    // base64
    function encode_fhe_value(value: Bytes): String;
    function decode_fhe_value(value: String): Bytes;

    // fhe_ops
    function add(a: Bytes, b: Bytes, type: String): Bytes;
    function sub(a: Bytes, b: Bytes, type: String): Bytes;
    function mul(a: Bytes, b: Bytes, type: String): Bytes;
    function div(a: Bytes, b: Bytes, type: String): Bytes;
    function rem(a: Bytes, b: Bytes, type: String): Bytes;
    function neg(a: Bytes, type: String): Bytes;
    function not(a: Bytes, type: String): Bytes;
    function and(a: Bytes, b: Bytes, type: String): Bytes;
    function or(a: Bytes, b: Bytes, type: String): Bytes;
    function xor(a: Bytes, b: Bytes, type: String): Bytes;

    // fhe
    function encrypt(val: Bytes, client_key: Bytes, data_type: String): Bytes;
    function decrypt(val: Bytes, client_key: Bytes, data_type: String): Bytes;
    function proven_encrypt(val: Bytes, public_key: Bytes, public_zk_params: Bytes, data_type: String): Bytes;

    // proven_fhe_ops
    function proven_add(a: Bytes, b: Bytes, type: String, public_zk_params: Bytes, public_key: Bytes): Bytes;
    function proven_sub(a: Bytes, b: Bytes, type: String, public_zk_params: Bytes, public_key: Bytes): Bytes;
    function proven_mul(a: Bytes, b: Bytes, type: String, public_zk_params: Bytes, public_key: Bytes): Bytes;
    function proven_div(a: Bytes, b: Bytes, type: String, public_zk_params: Bytes, public_key: Bytes): Bytes;
    function proven_rem(a: Bytes, b: Bytes, type: String, public_zk_params: Bytes, public_key: Bytes): Bytes;
    function proven_and(a: Bytes, b: Bytes, type: String, public_zk_params: Bytes, public_key: Bytes): Bytes;
    function proven_or(a: Bytes, b: Bytes, type: String, public_zk_params: Bytes, public_key: Bytes): Bytes;
    function proven_xor(a: Bytes, b: Bytes, type: String, public_zk_params: Bytes, public_key: Bytes): Bytes;
    function proven_neg(a: Bytes, type: String, public_zk_params: Bytes, public_key: Bytes): Bytes;
    function proven_not(a: Bytes, type: String, public_zk_params: Bytes, public_key: Bytes): Bytes;

    // serializer
    function from_i64(value: Number): Bytes;
    function from_u64(value: Number): Bytes;
    function to_i64(value: Bytes): Number;
    function to_u64(value: Bytes): Number;

    // http
    function create_fhe_header(method: String, zk_experiment?: Boolean): String;
    function encrypt_fhe_body(keys: Array<String>, data_type: String, data: String, client_key: Bytes): String;
    function decrypt_fhe_body(keys: Array<String>, data_type: String, data: String, client_key: Bytes): String;
    function set_server_key_to_json(server_key: Bytes, data: String): String;
    function set_public_key_to_json(public_key: Bytes, data: String): String;
    function set_public_zk_params_to_json(public_zk_params: Bytes, data: String): String;
    function get_fhe_value_from_json(key: String, data: String): Bytes;

    // other
    function set_server_key(server_key: Bytes): void;
    function get_public_zk_params(msg: Number, carry: Number): Bytes;
}

export class KeyGen {
    constructor() {}

    createConfig(params?: Bytes): Bytes {
        return addon.create_config(params);
    }
    generateClientKey(config: Bytes): Bytes {
        return addon.generate_client_key(config);
    }
    generateServerKey(client_key: Bytes): Bytes {
        return addon.generate_server_key(client_key);
    }
    generatePublicKey(client_key: Bytes): Bytes {
        return addon.generate_public_key(client_key);
    }
    generatePublicZkParams(max_num_message: Number, params?: Bytes): Bytes {
        return addon.generate_public_zk_params(max_num_message, params);
    }
}

export class Fhe {
    constructor() {}

    encrypt(val: Bytes, client_key: Bytes, data_type: String): Bytes {
        return addon.encrypt(val, client_key, data_type);
    }
    provenEncrypt(val: Bytes, public_key: Bytes, public_zk_params: Bytes, data_type: String): Bytes {
        return addon.proven_encrypt(val, public_key, public_zk_params, data_type);
    }
    decrypt(val: Bytes, client_key: Bytes, data_type: String): Bytes {
        return addon.decrypt(val, client_key, data_type);
    }
}

export class FheOps {
    constructor() {}

    add(a: Bytes, b: Bytes, type: String): Bytes {
        return addon.add(a, b, type);
    }
    sub(a: Bytes, b: Bytes, type: String): Bytes {
        return addon.sub(a, b, type);
    }
    mul(a: Bytes, b: Bytes, type: String): Bytes {
        return addon.mul(a, b, type);
    }
    div(a: Bytes, b: Bytes, type: String): Bytes {
        return addon.div(a, b, type);
    }
    rem(a: Bytes, b: Bytes, type: String): Bytes {
        return addon.rem(a, b, type);
    }
    neg(a: Bytes, type: String): Bytes {
        return addon.neg(a, type);
    }
    not(a: Bytes, type: String): Bytes {
        return addon.not(a, type);
    }
    and(a: Bytes, b: Bytes, type: String): Bytes {
        return addon.and(a, b, type);
    }
    or(a: Bytes, b: Bytes, type: String): Bytes {
        return addon.or(a, b, type);
    }
    xor(a: Bytes, b: Bytes, type: String): Bytes {
        return addon.xor(a, b, type);
    }
}

export class ProvenFheOps {
    constructor() {}

    add(a: Bytes, b: Bytes, type: String, publicZkParams: Bytes, publicKey: Bytes): Bytes {
        return addon.proven_add(a, b, type, publicZkParams, publicKey);
    }

    sub(a: Bytes, b: Bytes, type: String, publicZkParams: Bytes, publicKey: Bytes): Bytes {
        return addon.proven_sub(a, b, type, publicZkParams, publicKey);
    }
    mul(a: Bytes, b: Bytes, type: String, publicZkParams: Bytes, publicKey: Bytes): Bytes {
        return addon.proven_mul(a, b, type, publicZkParams, publicKey);
    }
    div(a: Bytes, b: Bytes, type: String, publicZkParams: Bytes, publicKey: Bytes): Bytes {
        return addon.proven_div(a, b, type, publicZkParams, publicKey);
    }
    rem(a: Bytes, b: Bytes, type: String, publicZkParams: Bytes, publicKey: Bytes): Bytes {
        return addon.proven_rem(a, b, type, publicZkParams, publicKey);
    }
    neg(a: Bytes, type: String, publicZkParams: Bytes, publicKey: Bytes): Bytes {
        return addon.proven_neg(a, type, publicZkParams, publicKey);
    }
    not(a: Bytes, type: String, publicZkParams: Bytes, publicKey: Bytes): Bytes {
        return addon.proven_not(a, type, publicZkParams, publicKey);
    }
    and(a: Bytes, b: Bytes, type: String, publicZkParams: Bytes, publicKey: Bytes): Bytes {
        return addon.proven_and(a, b, type, publicZkParams, publicKey);
    }
    or(a: Bytes, b: Bytes, type: String, publicZkParams: Bytes, publicKey: Bytes): Bytes {
        return addon.proven_or(a, b, type, publicZkParams, publicKey);
    }
    xor(a: Bytes, b: Bytes, type: String, publicZkParams: Bytes, publicKey: Bytes): Bytes {
        return addon.proven_xor(a, b, type, publicZkParams, publicKey);
    }
}

export class Base64 {
    constructor() {}

    encodeFheValue(value: Bytes): String {
        return addon.encode_fhe_value(value);
    }
    decodeFheValue(value: String): Bytes {
        return addon.decode_fhe_value(value);
    }
}

export class Serializer {
    constructor() {}

    fromI64(value: Number): Bytes {
        return addon.from_i64(value);
    }
    fromU64(value: Number): Bytes {
        return addon.from_u64(value);
    }
    toI64(value: Bytes): Number {
        return addon.to_i64(value);
    }
    toU64(value: Bytes): Number {
        return addon.to_u64(value);
    }
}

export function setServerKey(serverKey: Bytes): void {
    return addon.set_server_key(serverKey);
}

export function getPublicZkParams(msg: Number, carry: Number): Bytes {
    return addon.get_public_zk_params(msg, carry);
}

export function createFheHeader(method: String, zkExperiment?: Boolean): String {
    return addon.create_fhe_header(method, zkExperiment);
}
export function encryptFheBody(keys: Array<String>, dataType: String, data: JSON, clientKey: Bytes): String {
    let stringifiedData = JSON.stringify(data);
    return addon.encrypt_fhe_body(keys, dataType, stringifiedData, clientKey);
}
export function decryptFheBody(keys: Array<String>, dataType: String, data: JSON, clientKey: Bytes): String {
    let stringifiedData = JSON.stringify(data);
    return addon.decrypt_fhe_body(keys, dataType, stringifiedData, clientKey);
}
export function setServerKeyToJson(server_key: Bytes, data: JSON): String {
    let stringifiedData = JSON.stringify(data);
    return addon.set_server_key_to_json(server_key, stringifiedData);
}
export function setPublicKeyToJson(public_key: Bytes, data: JSON): String {
    let stringifiedData = JSON.stringify(data);
    return addon.set_public_key_to_json(public_key, stringifiedData);
}
export function setPublicZkParamsToJson(publicZkParams: Bytes, data: JSON): String {
    let stringifiedData = JSON.stringify(data);
    return addon.set_public_zk_params_to_json(publicZkParams, stringifiedData);
}
export function getFheValueFromJson(key: String, data: JSON): Bytes {
    let stringifiedData = JSON.stringify(data);
    return addon.get_fhe_value_from_json(key, stringifiedData);
}

export function uint8ArrayToBase64(arr) {
    return Buffer.from(arr).toString("base64");
}

export function base64ToUint8Array(base64String) {
    const buffer = Buffer.from(base64String, "base64");
    const uint8Array = new Uint8Array(buffer);
    return uint8Array;
}
