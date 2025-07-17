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
