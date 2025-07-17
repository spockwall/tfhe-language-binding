const fheHttp = require("fhe_js");
const express = require("express");
const bodyParser = require("body-parser");
const cors = require("cors");
const app = express();

const corsOptions = {
    origin: "*", //(https://your-client-app.com)
    optionsSuccessStatus: 200,
};

app.use(cors(corsOptions));
app.use(bodyParser.json({ limit: "5000mb" }));
app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());

// route
app.get("/", (req, res) => {
    res.send("Hello World");
});

app.post("/", async (req, res) => {
    let data = req.body;
    let fheOps = new fheHttp.FheOps();
    let encrypted_a = fheHttp.base64ToUint8Array(data["ciphertext1"]);
    let encrypted_b = fheHttp.base64ToUint8Array(data["ciphertext2"]);
    let server_key = fheHttp.base64ToUint8Array(data["serverKey"]);
    fheHttp.setServerKey(server_key);
    let encrypted_c = fheOps.add(encrypted_a, encrypted_b, "Uint64");
    let encoded_c = fheHttp.uint8ArrayToBase64(encrypted_c);
    let result = { result: encoded_c, status: "success" };
    res.json(result);
});

app.listen(3001, () => {
    console.log("Server is running on port 3001");
});
