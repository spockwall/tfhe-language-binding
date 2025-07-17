use pyo3;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;
use std::collections::HashMap;

// Helper function to convert PyAny to serde_json::Value
fn py_to_value<'py>(obj: Bound<'py, PyAny>) -> PyResult<Value> {
    // should be improved to comply with NorJsonValue
    if let Ok(val) = obj.extract::<i64>() {
        return Ok(Value::Number(val.into()));
    }
    if let Ok(val) = obj.extract::<u64>() {
        return Ok(Value::Number(val.into()));
    }
    if let Ok(val) = obj.extract::<bool>() {
        return Ok(Value::Bool(val));
    }
    if let Ok(val) = obj.extract::<String>() {
        return Ok(Value::String(val));
    }
    Err(pyo3::exceptions::PyTypeError::new_err("Unsupported type"))
}

#[pyfunction]
pub fn py_dict_to_json<'py>(data: Bound<'py, PyDict>) -> PyResult<String> {
    let mut map: HashMap<String, Value> = HashMap::new();

    // Iterate over the items in the PyDict and convert them
    for (key, value) in data {
        let key: String = key.extract()?;
        let value: Value = py_to_value(value)?;
        map.insert(key, value);
    }

    // Serialize the HashMap into a JSON string
    let json_str = serde_json::to_string(&map)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    Ok(json_str)
}
