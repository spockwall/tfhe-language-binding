use crate::assembly::{executor, parser};
use crate::configs::errors::AsmError;
use crate::configs::typing::FheType;
use crate::fhe_traits::serializable::FheValueSerializable;
use std::collections::HashMap;
use tfhe::{FheInt64, FheUint64};

fn deserialize_fhe_args<T>(args: &HashMap<String, Vec<u8>>) -> HashMap<String, T>
where
    T: FheValueSerializable,
{
    let mut fhe_args = HashMap::new();
    for (key, value) in args.iter() {
        let fhe_int64 = T::try_deserialize(value).unwrap();
        fhe_args.insert(key.clone(), fhe_int64);
    }
    fhe_args
}

pub fn execute_assembly(
    assembly: &str,
    args: HashMap<String, Vec<u8>>,
    data_type: &FheType,
) -> Result<Vec<u8>, AsmError> {
    let asm_parse_res = parser::parse_asm_from_str(assembly);
    let parsed_asm = match asm_parse_res {
        Ok(asm) => asm,
        Err(e) => panic!("Failed to parse assembly: {}", e),
    };

    match data_type {
        FheType::Int64 => {
            // iterate over args and turn vec<u8> into FheInt64
            let fhe_args = deserialize_fhe_args::<FheInt64>(&args);
            let result = executor::execute_asm_i64(&parsed_asm, &fhe_args);
            match result {
                Ok(res) => return Ok(res.try_serialize().expect("Failed to serialize")),
                Err(e) => return Err(e),
            }
        }
        FheType::Uint64 => {
            let fhe_args = deserialize_fhe_args::<FheUint64>(&args);
            let result = executor::execute_asm_u64(&parsed_asm, &fhe_args);
            match result {
                Ok(res) => return Ok(res.try_serialize().expect("Failed to serialize")),
                Err(e) => return Err(e),
            }
        }
    }
}
