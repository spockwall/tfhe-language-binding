from fhe_python import FheType, ProvenFheType

SerialData = list[int]

class KeyGenerator:
    client_key: SerialData
    server_key: SerialData
    __config: any
    def __init__(self) -> None: ...
    def init_keys(self) -> None: ...
    def generate_new_keys(self) -> None: ...
    def generate_public_zk_params(self) -> None: ...
    def get_client_key(self) -> SerialData: ...
    def get_server_key(self) -> SerialData: ...
    def get_public_key(self) -> SerialData: ...
    def __get_enc_path(self) -> str: ...
    def save_keys(self) -> None: ...
    def load_keys(self) -> None: ...

class ServerKeySetter:
    def set_server_key(self, server_key: SerialData) -> None: ...
    def decompress_server_key(self, server_key: SerialData) -> list[int]: ...

class Fhe:
    def __call__(
        self, client_key: SerialData, public_key: SerialData | None
    ) -> None: ...
    def encrypt(self, val: SerialData, data_type: FheType) -> SerialData: ...
    def proven_encrypt(
        self, val: SerialData, data_type: ProvenFheType
    ) -> SerialData: ...
    def decrypt(self, val: SerialData) -> SerialData: ...

class Serializer:
    def from_i64(self, value: int) -> SerialData: ...
    def from_u64(self, value: int) -> SerialData: ...
    def to_i64(self, value: SerialData) -> int: ...
    def to_u64(self, value: SerialData) -> int: ...

class FheOps:
    def set_server_key(self, server_key: list[int]) -> None: ...
    def add(self, a: SerialData, b: SerialData, data_type: str): ...
    def sub(self, a: SerialData, b: SerialData, data_type: str): ...
    def mul(self, a: SerialData, b: SerialData, data_type: str): ...
    def div(self, a: SerialData, b: SerialData, data_type: str): ...
    def and_(self, a: SerialData, b: SerialData, data_type: str): ...
    def or_(self, a: SerialData, b: SerialData, data_type: str): ...
    def xor(self, a: SerialData, b: SerialData, data_type: str): ...
    def shr(self, a: SerialData, b: SerialData, data_type: str): ...
    def shl(self, a: SerialData, b: SerialData, data_type: str): ...
    def not_(self, a: SerialData, data_type: str): ...
    # def neg(self, a: SerialData, data_type: str): ...
    def __getattr__(self, name):
        # Define a dictionary mapping method names to actual methods
        methods = {"and": self.and_, "or": self.or_, "not": self.not_}
        if name in methods:
            return methods[name]
        raise AttributeError(f"'FheOps' object has no attribute '{name}'")

class ProvenFheOps:
    def set_server_key(self, server_key: list[int]) -> None: ...
    def add(
        self,
        a: SerialData,
        b: SerialData,
        data_type: str,
        public_zk_param: SerialData,
        public_key: SerialData,
    ): ...
    def sub(
        self,
        a: SerialData,
        b: SerialData,
        data_type: str,
        public_zk_param: SerialData,
        public_key: SerialData,
    ): ...
    def mul(
        self,
        a: SerialData,
        b: SerialData,
        data_type: str,
        public_zk_param: SerialData,
        public_key: SerialData,
    ): ...
    def div(
        self,
        a: SerialData,
        b: SerialData,
        data_type: str,
        public_zk_param: SerialData,
        public_key: SerialData,
    ): ...
    def and_(
        self,
        a: SerialData,
        b: SerialData,
        data_type: str,
        public_zk_param: SerialData,
        public_key: SerialData,
    ): ...
    def or_(
        self,
        a: SerialData,
        b: SerialData,
        data_type: str,
        public_zk_param: SerialData,
        public_key: SerialData,
    ): ...
    def xor(
        self,
        a: SerialData,
        b: SerialData,
        data_type: str,
        public_zk_param: SerialData,
        public_key: SerialData,
    ): ...
    def shr(
        self,
        a: SerialData,
        b: SerialData,
        data_type: str,
        public_zk_param: SerialData,
        public_key: SerialData,
    ): ...
    def shl(
        self,
        a: SerialData,
        b: SerialData,
        data_type: str,
        public_zk_param: SerialData,
        public_key: SerialData,
    ): ...
    def not_(
        self,
        a: SerialData,
        data_type: str,
        public_zk_param: SerialData,
        public_key: SerialData,
    ): ...
    def neg(
        self,
        a: SerialData,
        data_type: str,
        public_zk_param: SerialData,
        public_key: SerialData,
    ): ...
    def __getattr__(self, name):
        # Define a dictionary mapping method names to actual methods
        methods = {"and": self.and_, "or": self.or_, "not": self.not_}
        if name in methods:
            return methods[name]
        raise AttributeError(f"'ProvenFheOps' object has no attribute '{name}'")

# FheType and ProvenFheType
def create_fhe_type(s: str) -> FheType: ...
def create_proven_fhe_type(s: str) -> ProvenFheType: ...

# public zk params
def get_public_zk_params(msg: int, carry: int) -> SerialData: ...

# http module
def create_fhe_header(method: str) -> str: ...
def decrypt_fhe_body(
    keys: dict[str, FheType], data: dict, client_key: SerialData
) -> dict: ...
def set_server_key_to_json(server_key: SerialData, data: dict) -> str: ...
def check_http_packet(packet: str) -> bool:
    """
    Check if the packet is a valid HTTP packet.
    packet is stringified JSON data.
    """
    ...

def get_fhe_value_from_json(key: str, data: dict) -> SerialData:
    """
    get the FHE value from the JSON data.
    result is serialized data, and should be deserailized to use.
    """
    ...

def execute_assembly(
    assembly: str, args: dict[str, SerialData], data_type: FheType
) -> SerialData:
    """
    Execute the assembly code compiled from python code. Args are
    the arguments of the assembly code and should be serialized.
    """
    ...
