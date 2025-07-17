const fhe_http = require("fhe_js");

describe("Test Encrypt/Decrypt", () => {
    beforeAll(() => {
        this.keyGen = new fhe_http.KeyGen();
        this.config = this.keyGen.createConfig();
        this.clientKey = this.keyGen.generateClientKey(this.config);
    });

    test("encrypt and decrypt i64", () => {
        let fhe = new fhe_http.Fhe();
        let serializer = new fhe_http.Serializer();
        let testCases = [0, 1, -1, 123, -123, 2147483647, -2147483647];
        for (let i = 0; i < testCases.length; i++) {
            let num = testCases[i];
            let plaintext = serializer.fromI64(num);
            let encrypted = fhe.encrypt(plaintext, this.clientKey, "Int64");
            let decrypted = fhe.decrypt(encrypted, this.clientKey, "Int64");
            let res = serializer.toI64(decrypted);

            expect(res).toBe(num);
        }
    });

    test("encrypt i64 fail for numbers over 32 bits", () => {
        let fhe = new fhe_http.Fhe();
        let serializer = new fhe_http.Serializer();
        let testCases = [2147483648, -2147483649, 12312312312123, 393993939393];
        for (let i = 0; i < testCases.length; i++) {
            let num = testCases[i];
            let plaintext = serializer.fromI64(num);
            let encrypted = fhe.encrypt(plaintext, this.clientKey, "Int64");
            let decrypted = fhe.decrypt(encrypted, this.clientKey, "Int64");
            let res = serializer.toI64(decrypted);
            expect(res).not.toBe(num);
        }
    });

    test("encrypt and decrypt u64", () => {
        let fhe = new fhe_http.Fhe();
        let serializer = new fhe_http.Serializer();
        let testCases = [0, 1, 123, 2147483647];
        for (let i = 0; i < testCases.length; i++) {
            let num = testCases[i];
            let plaintext = serializer.fromU64(num);
            let encrypted = fhe.encrypt(plaintext, this.clientKey, "Uint64");
            let decrypted = fhe.decrypt(encrypted, this.clientKey, "Uint64");
            let res = serializer.toU64(decrypted);

            expect(res).toBe(num);
        }
    });

    test("encrypt u64 fail for numbers over 32 bits and being negative", () => {
        let fhe = new fhe_http.Fhe();
        let serializer = new fhe_http.Serializer();
        let testCases = [2147483648, -2147483648, 12312312312123, -1];
        for (let i = 0; i < testCases.length; i++) {
            let num = testCases[i];
            let plaintext = serializer.fromU64(num);
            let encrypted = fhe.encrypt(plaintext, this.clientKey, "Uint64");
            let decrypted = fhe.decrypt(encrypted, this.clientKey, "Uint64");
            let res = serializer.toU64(decrypted);
            expect(res).not.toBe(num);
        }
    });
});
