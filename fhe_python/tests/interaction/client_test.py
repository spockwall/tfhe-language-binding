# client.py
import json
import requests
import fhe_http as py_fhe


def generate_keys():
    key_gen = py_fhe.KeyGenerator()
    key_gen.init_keys()
    client_key = key_gen.get_client_key()
    server_key = key_gen.get_server_key()
    return client_key, server_key


def decrypt(encrypted_num, client_key, data_type: str = "Int64"):
    serailizer = py_fhe.Serializer()
    fhe_value = py_fhe.create_fhe_type(data_type)
    fhe = py_fhe.Fhe(client_key)
    return serailizer.to_i64(fhe.decrypt(encrypted_num, fhe_value))


def send_post_request(url):
    header = json.loads(py_fhe.create_fhe_header("123"))
    client_key, server_key = generate_keys()
    data = {"a": 123123123, "b": 123}
    data_type = py_fhe.create_fhe_type("Int64")
    encrypt_json = py_fhe.encrypt_fhe_body(
        [("a", data_type), ("b", data_type)], data, client_key
    )
    encrypt_json = json.loads(encrypt_json)
    payload_str = py_fhe.set_server_key_to_json(server_key, encrypt_json)
    payload = json.loads(payload_str)
    response = requests.post(url, json=payload, headers=header)
    response = response.json()
    encrypted_c = py_fhe.decode_fhe_value(response["result"])
    c = decrypt(encrypted_c, client_key)
    assert c == 123123246


if __name__ == "__main__":
    server_url = "http://localhost:8000"
    send_post_request(server_url)
