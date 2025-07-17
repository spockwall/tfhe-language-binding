const fhe_http = require("fhe_js");

describe("Test KeyGen", () => {
    test("Should generate keys", () => {
        let keyGen = new fhe_http.KeyGen();
        let config = keyGen.createConfig();
        let clientKey = keyGen.generateClientKey(config);
        let serverKey = keyGen.generateServerKey(clientKey);
        let publicKey = keyGen.generatePublicKey(clientKey);

        expect(clientKey).not.toBeNull();
        expect(clientKey).toBeInstanceOf(Uint8Array);

        expect(serverKey).not.toBeNull();
        expect(serverKey).toBeInstanceOf(Uint8Array);

        expect(publicKey).not.toBeNull();
        expect(publicKey).toBeInstanceOf(Uint8Array);
    });
});