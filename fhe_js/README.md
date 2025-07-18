# Fhe-Language-Binding-JS

## Introduce

- This project offers a JavaScript interface for using [tfhe-rs](https://github.com/zama-ai/tfhe-rs). Additionally, a zk-experimental version is supported, enabling the server to verify encrypted values before computation begins, and assembly code execution allows clients to define the computation process/
- Currently, gpu acceleration of tfhe-rs is not yet available in this project.
- This package can only be run on `Node.js`.

## Fully homomorphic encryption (FHE)

FHE computation enables clients to delegate computation tasks to servers. Clients use a client key which is generated by themselves to encrypt the values. After encryption, clients send the encrypted values and a server key to the servers. Once the server key is received and set by the servers, the servers can execute the FHE computation on the ciphertexts. The computation result remains encrypted and is supposed to be sent back to the clients, who can then decrypt it to obtain the result in plaintext.

## Required

- `Node.js^16`

## How to use

### Run Tests

#### Execute tests in __tests__

```shellscript
$ cd fhe-language-binding/fhe_js
$ npm install
$ npm run build
$ npm run test-js
```

#### Test packaging at local

```shellscript
$ cd fhe-language-binding/fhe_js
$ npm install
$ npm run build
$ npm link

// change to Node.js application dir
$ cd <nodejs project path>
$ npm link fhe_js

// do something here ... 
```

## Details of usage

### Supported

- operations:
  - `add`, `sub`, `mul`, `div`, `rem`, `and`, `or`, `xor`, `shr`, `shl`, `not`, `and`, `neg`
- FheType:
  - `Int64`
  - `Uint64`

### Example usage

```javascript
const fhe_http = require("fhe_js");

const keyGen = new fhe_http.KeyGen();
const fhe = new fhe_http.Fhe();
const fhe_ops = new fhe_http.FheOps();
const serializer = new fhe_http.Serializer();

let config = keyGen.createConfig();
let clientKey = keyGen.generateClientKey(config);
let serverKey = keyGen.generateServerKey(clientKey);
fhe_http.set_server_key(serverKey);
let cases = [
    { a: 123, b: 24 },
    { a: -123, b: -24 },
    { a: 2147483647, b: -12324 },
];
for (let i = 0; i < cases.length; i++) {
    let plaintext_a = serializer.fromI64(cases[i]["a"]);
    let plaintext_b = serializer.fromI64(cases[i]["b"]);
    let encrypted_a = fhe.encrypt(plaintext_a, clientKey, "Int64");
    let encrypted_b = fhe.encrypt(plaintext_b, clientKey, "Int64");
    let encrypted_res = fhe_ops.add(encrypted_a, encrypted_b, "Int64");
    let decrypted_res = fhe.decrypt(encrypted_res, clientKey, "Int64");
    let res = serializer.toI64(decrypted_res);
    console.log(res);
}

```

## Interaction via Http

### Decription

This nodejs package also provides some functions for http interaction. For example: setting the header to comfirm the fhe protocol, and some json related operation for both body and header (see below exmaple)

### Example usage

#### Execution

```shellscript
$ cd fhe-language-binding/fhe_js
$ npm run build
$ cd __tests__/interaction/
$ npm install

// machine 1
$ npm run start

// machine 2
$ node client.js
```

#### Client Side

```javascript
const fheHttp = require("fhe_js");
const path = require("path");

// send http request to server

function generate_keys() {
    let keyGen = new fheHttp.KeyGen();
    let config = keyGen.createConfig();
    let clientKey = keyGen.generateClientKey(config);
    let serverKey = keyGen.generateServerKey(clientKey);
    return { clientKey, serverKey };
}

function decrypt(encryptedNum, clientKey, dataType = "Int64") {
    let serializer = new fheHttp.Serializer();
    let fhe = new fheHttp.Fhe();
    let res = fhe.decrypt(encryptedNum, clientKey, dataType);
    return serializer.toI64(res);
}

function sendPostRequest(url) {
    let header = JSON.parse(fheHttp.createFheHeader("123"));

    let { clientKey, serverKey } = generate_keys();
    let data = { a: 1223123, b: 123123 };
    let payload_str = fheHttp.encryptFheBody(["a", "b"], "Int64", data, clientKey);
    let payload = JSON.parse(payload_str);
    payload_str = fheHttp.setServerKeyToJson(serverKey, payload);
    delete header["content-encoding"];
    (async () => {
        try {
            const res = await fetch("http://localhost:3000", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    ...header,
                },
                body: payload_str,
            });
            console.log("Status Code:", res.status);

            const data = await res.json();
            let base64 = new fheHttp.Base64();
            const c = base64.decodeFheValue(data["result"]);
            console.log("Decrypted Result:", decrypt(c, clientKey));
        } catch (err) {
            console.log(err.message); //can be console.error
        }
    })();
}

sendPostRequest();
```

#### Server Side

```javascript
const fheHttp = require("fhe_js");
const express = require("express");
const bodyParser = require("body-parser");
const app = express();

app.use(bodyParser.json({ limit: "5000mb" }));
app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());

// route
app.get("/", (req, res) => {
    res.send("Hello World");
});
app.post("/", async (req, res) => {
    let data = req.body;
    let base64 = new fheHttp.Base64();
    let fheOps = new fheHttp.FheOps();
    let encrypted_a = base64.decodeFheValue(data["a"]);
    let encrypted_b = base64.decodeFheValue(data["b"]);
    let server_key = base64.decodeFheValue(data["server_key"]);
    fheHttp.setServerKey(server_key);
    let encrypted_c = fheOps.add(encrypted_a, encrypted_b, "Int64");
    let encoded_c = base64.encodeFheValue(encrypted_c);
    let result = { result: encoded_c, status: "success" };
    res.json(result);
});

app.listen(3000, () => {
    console.log("Server is running on port 3000");
});
```
