import json
import fastapi
import uvicorn
from fastapi import Body
from pydantic import BaseModel
import fhe_http as py_fhe


app = fastapi.FastAPI()


class Addition(BaseModel):
    a: str
    b: str
    server_key: str


def encrypt(num: int, client_key, data_type: str = "Int64"):
    serailizer = py_fhe.Serializer()
    fhe_value = py_fhe.create_fhe_type(data_type)
    fhe = py_fhe.Fhe(client_key)
    return fhe.encrypt(serailizer.from_i64(num), fhe_value)


def set_server_key(server_key):
    sk_setter = py_fhe.ServerKeySetter()
    decompressed_server_key = sk_setter.decompress_server_key(server_key)
    sk_setter.set_server_key(decompressed_server_key)


@app.post("/")
async def post_request(data: Addition = Body(...)):
    print("Received request")
    data_type = py_fhe.create_fhe_type("Int64")
    data_json = json.loads(data.model_dump_json())
    encrypted_a = py_fhe.decode_fhe_value(data_json["a"])
    encrypted_b = py_fhe.decode_fhe_value(data_json["b"])
    server_key = py_fhe.decode_fhe_value(data_json["server_key"])
    set_server_key(server_key)
    fhe_ops = py_fhe.FheOps()
    encrypted_c = fhe_ops.add(encrypted_a, encrypted_b, data_type)
    encoded_c = py_fhe.encode_fhe_value(encrypted_c)
    return {"result": encoded_c, "status": "success"}


if __name__ == "__main__":
    uvicorn.run("server_test:app", host="localhost", port=8000, reload=True)
