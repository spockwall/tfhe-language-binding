import fhe_http as py_fhe
from fhe_http.assembler.assembler import Assembler


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


def encrypt(num: int, client_key, data_type: str = "Uint64"):
    serailizer = py_fhe.Serializer()
    fhe_value = py_fhe.create_fhe_type(data_type)
    fhe = py_fhe.Fhe(client_key)
    return fhe.encrypt(serailizer.from_i64(num), fhe_value)


def decrypt(encrypted_num, client_key, data_type: str = "Uint64"):
    serailizer = py_fhe.Serializer()
    fhe_value = py_fhe.create_fhe_type(data_type)
    fhe = py_fhe.Fhe(client_key)
    return serailizer.to_i64(fhe.decrypt(encrypted_num, fhe_value))


def get_asm_code():
    assembler = Assembler()

    @assembler.code_wrapper
    def operation(i, j, two, three):
        a = i + j
        b = a >> two
        c = two << three
        d = c - b
        return d

    return "\n".join(operation.assembly)


if __name__ == "__main__":
    assembly = get_asm_code()
    client_key, server_key = generate_keys()
    set_server_key(server_key)

    encrypted = py_fhe.execute_assembly(
        assembly,
        {
            "i": encrypt(25, client_key),
            "j": encrypt(21, client_key),
            "two": encrypt(2, client_key),
            "three": encrypt(3, client_key),
        },
        py_fhe.create_fhe_type("Uint64"),
    )
    decrypted = decrypt(encrypted, client_key)
    print(decrypted)
