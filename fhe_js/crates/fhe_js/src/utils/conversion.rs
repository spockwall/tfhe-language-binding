use crate::configs::error::ConversionError;
use fhe_core::serde_json;
use neon::prelude::*;
use std::collections::HashMap;

#[allow(unused)]
pub(crate) fn vec_u8_to_array<'a, C: Context<'a>>(
    cx: &mut C,
    vec: &Vec<u8>,
) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as usize);

    for (i, s) in vec.iter().enumerate() {
        let v = cx.number(*s);
        a.set(cx, i as u32, v)?;
    }
    Ok(a)
}

#[allow(unused)]
pub(crate) fn array_to_vec_u8<'a, C: Context<'a>>(cx: &mut C, arr: Handle<JsArray>) -> Vec<u8> {
    let vec = arr.to_vec(cx).unwrap();

    vec.iter()
        .map(|v| {
            let num: Handle<'a, JsNumber> = v.downcast(cx).unwrap();
            num.value(cx) as u8
        })
        .collect()
}

#[allow(unused)]
pub(crate) fn array_to_vec_string<'a, C: Context<'a>>(
    cx: &mut C,
    arr: Handle<JsArray>,
) -> Vec<String> {
    let vec = arr.to_vec(cx).unwrap();

    vec.iter()
        .map(|v| {
            let str: Handle<'a, JsString> = v.downcast(cx).unwrap();
            str.value(cx)
        })
        .collect()
}

#[allow(unused)]
pub(crate) fn vec_string_to_array<'a, C: Context<'a>>(
    cx: &mut C,
    vec: &Vec<String>,
) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as usize);

    for (i, s) in vec.iter().enumerate() {
        let v = cx.string(s);
        a.set(cx, i as u32, v)?;
    }
    Ok(a)
}

#[allow(unused)]
pub(crate) fn object_to_json_str<'a, C: Context<'a>>(
    cx: &mut C,
    data: Handle<JsObject>,
) -> Result<String, ConversionError> {
    let res_str = data
        .to_string(cx)
        .map_err(|e| ConversionError::JsObjectConversionError(e.to_string()))?;
    Ok(res_str.value(cx))
}

#[allow(unused)]
pub(crate) fn hashmap_to_json_str(data: &HashMap<String, String>) -> Result<String, String> {
    serde_json::to_string(&data).map_err(|e| e.to_string())
}
