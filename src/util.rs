use js_sys::Array;
use wasm_bindgen::JsValue;


pub fn set_local_storage(key: &str, value: &str) {
    match web_sys::window().unwrap().local_storage().unwrap() {
        Some(local_storage) => {
            local_storage.set_item(key, value).unwrap();
        },
        _ => {}
    }
}

pub fn js_array(values: &[&str]) -> Array {
    return values.into_iter()
        .map(|x| JsValue::from_str(x))
        .collect::<Array>();
}