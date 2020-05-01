use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(raw_module = "/static/jsm/assembler/logger.js")]
extern {
    pub type Logger;

    #[wasm_bindgen(static_method_of = Logger, js_name = setCurrentLine)]
    pub fn set_current_line(line_num: usize);

    #[wasm_bindgen(static_method_of = Logger, js_name = setCurrentLine)]
    pub fn set_current_line_null();

    #[wasm_bindgen(static_method_of = Logger, js_name = setCurrentLine)]
    pub fn set_current_line_str(txt: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = beginInfo)]
    pub fn begin_info();

    #[wasm_bindgen(static_method_of = Logger, js_name = beginWarn)]
    pub fn begin_warn();

    #[wasm_bindgen(static_method_of = Logger, js_name = beginErr)]
    pub fn begin_err();

    #[wasm_bindgen(static_method_of = Logger, js_name = write)]
    pub fn write_str(text: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = write)]
    pub fn write_i32(num: i32);

    #[wasm_bindgen(static_method_of = Logger, js_name = writeCode)]
    pub fn write_code(code: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = writeCode)]
    pub fn write_code_i32(code: i32);

    #[wasm_bindgen(static_method_of = Logger, js_name = endMessage)]
    pub fn end_msg();

    #[wasm_bindgen(static_method_of = Logger, js_name = genericMessage)]
    fn generic_message(kind: &str, msg: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = genericExplainedCode)]
    fn generic_explained_code(kind: &str, msg1: &str, code: &str, msg2: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = genericExplainedCode)]
    fn generic_explained_code_i32(kind: &str, msg1: &str, code: i32, msg2: &str);
}

#[inline(always)]
pub fn err_msg(msg: &str) {
    Logger::generic_message("err", msg)
}

#[inline(always)]
pub fn warn_msg(msg: &str) {
    Logger::generic_message("warn", msg)
}

#[inline(always)]
pub fn info_msg(msg: &str) {
    Logger::generic_message("info", msg)
}


#[inline(always)]
pub fn err_code(msg1: &str, code: &str, msg2: &str) {
    Logger::generic_explained_code("err", msg1, code, msg2)
}

#[inline(always)]
pub fn warn_code(msg1: &str, code: &str, msg2: &str) {
    Logger::generic_explained_code("warn", msg1, code, msg2)
}

#[inline(always)]
pub fn info_code(msg1: &str, code: &str, msg2: &str) {
    Logger::generic_explained_code("info", msg1, code, msg2)
}

#[inline(always)]
pub fn info_code_i32(msg1: &str, code: i32, msg2: &str) {
    Logger::generic_explained_code_i32("info", msg1, code, msg2)
}