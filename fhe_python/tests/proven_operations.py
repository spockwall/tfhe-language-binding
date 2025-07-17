import unittest
import fhe_http as py_fhe
from fhe_http import create_proven_fhe_type
from fhe_http import get_public_zk_params


def generate_keys():
    key_gen = py_fhe.KeyGenerator()
    key_gen.init_keys()
    client_key = key_gen.get_client_key()
    server_key = key_gen.get_server_key()
    public_key = key_gen.get_public_key()
    return client_key, server_key, public_key


def set_server_key(server_key):
    sk_setter = py_fhe.ServerKeySetter()
    decompressed_server_key = sk_setter.decompress_server_key(server_key)
    sk_setter.set_server_key(decompressed_server_key)


def proven_encrypt(num: int, client_key, public_key, data_type, public_zk_params):
    serailizer = py_fhe.Serializer()
    proven_fhe_value = create_proven_fhe_type(data_type)
    fhe = py_fhe.Fhe(client_key, public_key)
    if data_type == "ProvenUint64":
        return fhe.proven_encrypt(
            serailizer.from_u64(num), proven_fhe_value, public_zk_params
        )
    return fhe.proven_encrypt(
        serailizer.from_i64(num), proven_fhe_value, public_zk_params
    )


def decrypt(encrypted_num, client_key, data_type: str = "ProvenInt64"):
    serailizer = py_fhe.Serializer()
    proven_fhe_value = create_proven_fhe_type(data_type)
    fhe = py_fhe.Fhe(client_key)
    if data_type == "ProvenUint64":
        return serailizer.to_u64(fhe.decrypt(encrypted_num, proven_fhe_value))
    return serailizer.to_i64(fhe.decrypt(encrypted_num, proven_fhe_value))


class TestFheOps(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.proven_ops = py_fhe.ProvenFheOps()
        cls.public_zk_params = get_public_zk_params(msg=2, carry=2)
        cls.num = [2500, 21]
        cls.encrypted_num_i64 = cls.get_encrypted_num()
        cls.encrypted_num_u64 = cls.get_encrypted_num(data_type="ProvenUint64")
        cls.client_key, cls.server_key, cls.public_key = generate_keys()
        set_server_key(cls.server_key)

    def get_encrypted_num(self, num=2, data_type="ProvenInt64"):
        return [
            proven_encrypt(
                self.num[i],
                self.client_key,
                self.public_key,
                data_type,
                self.public_zk_params,
            )
            for i in range(num)
        ]

    def exec_binary_operation(self, encrypted_a, encrypted_b, method, type):
        proven_fhe_value = create_proven_fhe_type(type)
        return method(
            encrypted_a,
            encrypted_b,
            proven_fhe_value,
            self.public_zk_params,
            self.public_key,
        )

    def exec_unary_operation(self, encrypted_a, method, type):
        proven_fhe_value = create_proven_fhe_type(type)
        return method(
            encrypted_a, proven_fhe_value, self.public_zk_params, self.public_key
        )

    def test_proven_add_encrypted_i64(self, data_type="ProvenInt64"):
        encrypted = self.encrypted_num_i64
        encrypted_c = self.exec_binary_operation(
            encrypted[0], encrypted[1], self.proven_ops.add, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.num[0] + self.num[1])

    def test_proven_sub_encrypted_i64(self, data_type="ProvenInt64"):
        encrypted = self.encrypted_num_i64
        encrypted_c = self.exec_binary_operation(
            encrypted[0], encrypted[1], self.proven_ops.sub, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.num[0] - self.num[1])

    def test_proven_mul_encrypted_i64(self, data_type="ProvenInt64"):
        encrypted = self.encrypted_num_i64
        encrypted_c = self.exec_binary_operation(
            encrypted[0], encrypted[1], self.proven_ops.mul, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.num[0] * self.num[1])

    def test_proven_div_encrypted_i64(self, data_type="ProvenInt64"):
        encrypted = self.encrypted_num_i64
        encrypted_c = self.exec_binary_operation(
            encrypted[0], encrypted[1], self.proven_ops.div, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.num[0] // self.num[1])

    def test_proven_and_encrypted_i64(self, data_type="ProvenInt64"):
        encrypted = self.encrypted_num_i64
        encrypted_c = self.exec_binary_operation(
            encrypted[0], encrypted[1], getattr(self.proven_ops, "and"), data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.num[0] & self.num[1])

    def test_proven_or_encrypted_i64(self, data_type="ProvenInt64"):
        encrypted = self.encrypted_num_i64
        encrypted_c = self.exec_binary_operation(
            encrypted[0], encrypted[1], getattr(self.proven_ops, "or"), data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.num[0] | self.num[1])

    def test_proven_xor_encrypted_i64(self, data_type="ProvenInt64"):
        encrypted = self.encrypted_num_i64
        encrypted_c = self.exec_binary_operation(
            encrypted[0], encrypted[1], self.proven_ops.xor, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.num[0] ^ self.num[1])

    def test_proven_not_encrypted_i64(self, data_type="ProvenInt64"):
        encrypted = self.encrypted_num_i64
        encrypted_c = self.exec_unary_operation(
            encrypted[0], getattr(self.proven_ops, "not"), data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, ~self.num[0])

    def test_proven_neg_encrypted_i64(self, data_type="ProvenInt64"):
        encrypted = self.encrypted_num_i64
        encrypted_c = self.exec_unary_operation(
            encrypted[0], getattr(self.proven_ops, "not"), data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, -self.num[0])

    def test_proven_shr_encrypted_u64(self, data_type="ProvenUint64"):
        encrypted = self.encrypted_num_u64
        encrypted_c = self.exec_binary_operation(
            encrypted[0], encrypted[1], self.proven_ops.shr, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.num[0] >> self.num[1])

    def test_proven_shl_encrypted_u64(self, data_type="ProvenUint64"):
        encrypted = self.encrypted_num_u64
        encrypted_c = self.exec_binary_operation(
            encrypted[0], encrypted[1], self.proven_ops.shl, data_type
        )
        c = decrypt(encrypted_c, self.client_key, data_type)
        self.assertEqual(c, self.num[0] << self.num[1])


if __name__ == "__main__":
    unittest.main()
