import fhe_http as py_fhe
import unittest


def generate_keys():
    key_gen = py_fhe.KeyGenerator()
    key_gen.init_keys()
    client_key = key_gen.get_client_key()
    server_key = key_gen.get_server_key()
    return client_key, server_key


def set_server_key(server_key):
    sk_setter = py_fhe.ServerKeySetter()
    decompressed_server_key = sk_setter.decompress_server_key(server_key)
    sk_setter.set_server_key(decompressed_server_key)


def encrypt(num: int, client_key, data_type: str = "Int64"):
    serailizer = py_fhe.Serializer()
    fhe_value = py_fhe.create_fhe_type(data_type)
    fhe = py_fhe.Fhe(client_key)
    return fhe.encrypt(serailizer.from_i64(num), fhe_value)


def decrypt(encrypted_num, client_key, data_type: str = "Int64"):
    serailizer = py_fhe.Serializer()
    fhe_value = py_fhe.create_fhe_type(data_type)
    fhe = py_fhe.Fhe(client_key)
    return serailizer.to_i64(fhe.decrypt(encrypted_num, fhe_value))


def exec_binary_operation(encrypted_a, encrypted_b, method, type):
    fhe_value = py_fhe.create_fhe_type(type)
    return method(encrypted_a, encrypted_b, fhe_value)


def exec_unary_operation(encrypted_a, method, type):
    fhe_value = py_fhe.create_fhe_type(type)
    return method(encrypted_a, fhe_value)


class TestFheOps(unittest.TestCase):

    def __init__(self, methodName: str = "runTest") -> None:
        super().__init__(methodName=methodName)
        self.ops = py_fhe.FheOps()
        self.a = 25
        self.b = 21

    @classmethod
    def setUpClass(cls):
        cls.client_key, cls.server_key = generate_keys()
        set_server_key(cls.server_key)

    def test_add_encrypted_i64(self, data_type="Int64"):
        encrypted_a = encrypt(self.a, self.client_key, data_type)
        encrypted_b = encrypt(self.b, self.client_key, data_type)
        encrypted_c = exec_binary_operation(
            encrypted_a, encrypted_b, self.ops.add, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.a + self.b)

    def test_sub_encrypted_i64(self, data_type="Int64"):
        encrypted_a = encrypt(self.a, self.client_key, data_type)
        encrypted_b = encrypt(self.b, self.client_key, data_type)
        encrypted_c = exec_binary_operation(
            encrypted_a, encrypted_b, self.ops.sub, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.a - self.b)

    def test_mul_encrypted_i64(self, data_type="Int64"):
        encrypted_a = encrypt(self.a, self.client_key, data_type)
        encrypted_b = encrypt(self.b, self.client_key, data_type)
        encrypted_c = exec_binary_operation(
            encrypted_a, encrypted_b, self.ops.mul, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.a * self.b)

    def test_div_encrypted_i64(self, data_type="Int64"):
        encrypted_a = encrypt(self.a, self.client_key, data_type)
        encrypted_b = encrypt(self.b, self.client_key, data_type)
        encrypted_c = exec_binary_operation(
            encrypted_a, encrypted_b, self.ops.div, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.a // self.b)

    def test_and_encrypted_i64(self, data_type="Int64"):
        encrypted_a = encrypt(self.a, self.client_key, data_type)
        encrypted_b = encrypt(self.b, self.client_key, data_type)
        encrypted_c = exec_binary_operation(
            encrypted_a, encrypted_b, getattr(self.ops, "and"), data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.a & self.b)

    def test_or_encrypted_i64(self, data_type="Int64"):
        encrypted_a = encrypt(self.a, self.client_key, data_type)
        encrypted_b = encrypt(self.b, self.client_key, data_type)
        encrypted_c = exec_binary_operation(
            encrypted_a, encrypted_b, getattr(self.ops, "or"), data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.a | self.b)

    def test_xor_encrypted_i64(self, data_type="Int64"):
        encrypted_a = encrypt(self.a, self.client_key, data_type)
        encrypted_b = encrypt(self.b, self.client_key, data_type)
        encrypted_c = exec_binary_operation(
            encrypted_a, encrypted_b, self.ops.xor, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.a ^ self.b)

    def test_not_encrypted_i64(self, data_type="Int64"):
        encrypted_a = encrypt(self.a, self.client_key, data_type)
        encrypted_c = exec_unary_operation(
            encrypted_a, getattr(self.ops, "not"), data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, ~self.a)

    def test_neg_encrypted_i64(self, data_type="Int64"):
        encrypted_a = encrypt(self.a, self.client_key, data_type)
        encrypted_c = exec_unary_operation(encrypted_a, self.ops.neg, data_type)
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, -self.a)

    def test_shr_encrypted_u64(self, data_type="Uint64"):
        encrypted_a = encrypt(self.a, self.client_key, data_type)
        encrypted_b = encrypt(2, self.client_key, data_type)
        encrypted_c = exec_binary_operation(
            encrypted_a, encrypted_b, self.ops.shr, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.a >> 2)

    def test_shl_encrypted_u64(self, data_type="Uint64"):
        encrypted_a = encrypt(self.a, self.client_key, data_type)
        encrypted_b = encrypt(2, self.client_key, data_type)
        encrypted_c = exec_binary_operation(
            encrypted_a, encrypted_b, self.ops.shl, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.a << 2)


if __name__ == "__main__":
    unittest.main()
