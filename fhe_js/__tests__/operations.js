const fhe_http = require("fhe_js");

describe("Test Fhe Int64 Operation", () => {
    const encrypt_a_and_b = (a, b) => {
        let plaintext_a = this.serializer.fromI64(a);
        let plaintext_b = this.serializer.fromI64(b);
        let encrypted_a = this.fhe.encrypt(plaintext_a, this.clientKey, "Int64");
        let encrypted_b = this.fhe.encrypt(plaintext_b, this.clientKey, "Int64");
        return [encrypted_a, encrypted_b];
    };

    beforeAll(() => {
        this.keyGen = new fhe_http.KeyGen();
        this.fhe = new fhe_http.Fhe();
        this.fhe_ops = new fhe_http.FheOps();
        this.serializer = new fhe_http.Serializer();
        this.config = this.keyGen.createConfig();
        this.clientKey = this.keyGen.generateClientKey(this.config);
        this.serverKey = this.keyGen.generateServerKey(this.clientKey);
        this.publicKey = this.keyGen.generatePublicKey(this.clientKey);
        fhe_http.setServerKey(this.serverKey);
    });

    test("Fhe i64 Addition", () => {
        let testCases = [
            // normal case
            {
                a: 123,
                b: 24,
            },

            // negative case
            {
                a: -123,
                b: -24,
            },

            // edge case
            {
                a: 2147483647,
                b: -12324,
            },
        ];

        for (let i = 0; i < testCases.length; i++) {
            let operands = encrypt_a_and_b(testCases[i].a, testCases[i].b);
            let encrypted_res = this.fhe_ops.add(operands[0], operands[1], "Int64");
            let decrypted_res = this.fhe.decrypt(encrypted_res, this.clientKey, "Int64");
            let res = this.serializer.toI64(decrypted_res);
            expect(res).toBe(testCases[i].a + testCases[i].b);
        }
    });

    test("Fhe i64 Substraction", () => {
        let testCases = [
            // normal case
            {
                a: 123,
                b: 24,
            },

            // negative case
            {
                a: -123,
                b: -24,
            },

            // edge case
            {
                a: 2147483647,
                b: 12324,
            },
        ];
        for (let i = 0; i < testCases.length; i++) {
            let operands = encrypt_a_and_b(testCases[i].a, testCases[i].b);
            let encrypted_res = this.fhe_ops.sub(operands[0], operands[1], "Int64");
            let decrypted_res = this.fhe.decrypt(encrypted_res, this.clientKey, "Int64");
            let res = this.serializer.toI64(decrypted_res);
            expect(res).toBe(testCases[i].a - testCases[i].b);
        }
    });
});

describe("Test Fhe Uint64 Operation", () => {
    const encrypt_a_and_b = (a, b) => {
        let plaintext_a = this.serializer.fromI64(a);
        let plaintext_b = this.serializer.fromI64(b);
        let encrypted_a = this.fhe.encrypt(plaintext_a, this.clientKey, "Uint64");
        let encrypted_b = this.fhe.encrypt(plaintext_b, this.clientKey, "Uint64");
        return [encrypted_a, encrypted_b];
    };
    beforeAll(() => {
        this.keyGen = new fhe_http.KeyGen();
        this.fhe = new fhe_http.Fhe();
        this.fhe_ops = new fhe_http.FheOps();
        this.serializer = new fhe_http.Serializer();
        this.config = this.keyGen.createConfig();
        this.clientKey = this.keyGen.generateClientKey(this.config);
        this.serverKey = this.keyGen.generateServerKey(this.clientKey);
        this.publicKey = this.keyGen.generatePublicKey(this.clientKey);
        fhe_http.setServerKey(this.serverKey);
    });

    test("Fhe u64 Addition", () => {
        let testCases = [
            {
                a: 123,
                b: 24,
            },
            {
                a: 2147483647,
                b: 0,
            },
            {
                a: 0,
                b: 2424,
            },
        ];
        for (let i = 0; i < testCases.length; i++) {
            let operands = encrypt_a_and_b(testCases[i].a, testCases[i].b);
            let encrypted_res = this.fhe_ops.add(operands[0], operands[1], "Uint64");
            let decrypted_res = this.fhe.decrypt(encrypted_res, this.clientKey, "Uint64");
            let res = this.serializer.toI64(decrypted_res);
            expect(res).toBe(testCases[i].a + testCases[i].b);
        }
    });

    test("Fhe u64 Substraction", () => {
        let testCases = [
            {
                a: 123,
                b: 24,
            },
            {
                a: 2147483647,
                b: 0,
            },
            {
                a: 0,
                b: 0,
            },
        ];
        for (let i = 0; i < testCases.length; i++) {
            let operands = encrypt_a_and_b(testCases[i].a, testCases[i].b);
            let encrypted_res = this.fhe_ops.sub(operands[0], operands[1], "Uint64");
            let decrypted_res = this.fhe.decrypt(encrypted_res, this.clientKey, "Uint64");
            let res = this.serializer.toI64(decrypted_res);
            expect(res).toBe(testCases[i].a - testCases[i].b);
        }
    });
});
