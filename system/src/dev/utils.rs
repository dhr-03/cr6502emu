use js_sys::Map;
use wasm_bindgen::JsValue;

#[inline(always)]
pub fn js_map_add_entry_f64<T: Into<f64>>(pkg: &Map, key: &str, value: T) {
    pkg.set(&JsValue::from_str(key), &JsValue::from_f64(value.into()));
}

#[inline(always)]
pub fn js_map_add_entry_bool(pkg: &Map, key: &str, value: bool) {
    pkg.set(&JsValue::from_str(key), &JsValue::from_bool(value));
}
