use wasm_bindgen::prelude::wasm_bindgen;
use super::lang::LoggerMessage;

#[wasm_bindgen(raw_module = "../shared/logger.js")]
extern {
    pub type Logger;

    #[wasm_bindgen(static_method_of = Logger, js_name = setCurrentLine)]
    pub fn set_current_line(line_num: usize);

    #[wasm_bindgen(static_method_of = Logger, js_name = setCurrentLine)]
    pub fn set_current_line_null();

    #[wasm_bindgen(static_method_of = Logger, js_name = setCurrentLine)]
    pub fn set_current_line_str(txt: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = writeCode)]
    pub fn write_code(code: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = writeCode)]
    pub fn write_code_i32(code: i32);

    #[wasm_bindgen(static_method_of = Logger, js_name = endMessage)]
    pub fn end_msg();

    #[wasm_bindgen(static_method_of = Logger, js_name = infoMessage)]
    pub fn info_msg(id: LoggerMessage);

    #[wasm_bindgen(static_method_of = Logger, js_name = warnMessage)]
    pub fn warn_msg(id: LoggerMessage);

    #[wasm_bindgen(static_method_of = Logger, js_name = errMessage)]
    pub fn err_msg(id: LoggerMessage);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedInfo)]
    pub fn explained_info(id: LoggerMessage, code: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedInfo)]
    pub fn explained_info_2(id: LoggerMessage, code_1: &str, code_2: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedInfo)]
    pub fn explained_info_i32(id: LoggerMessage, code: i32);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedInfo)]
    pub fn explained_info_i32_2(id: LoggerMessage, code_1: i32, code_2: i32);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedWarn)]
    pub fn explained_warn(id: LoggerMessage, code: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedWarn)]
    pub fn explained_warn_2(id: LoggerMessage, code_1: &str, code_2: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedWarn)]
    pub fn explained_warn_i32(id: LoggerMessage, code: i32);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedWarn)]
    pub fn explained_warn_i32_2(id: LoggerMessage, code_1: i32, code_2: i32);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedErr)]
    pub fn explained_err(id: LoggerMessage, code: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedErr)]
    pub fn explained_err_2(id: LoggerMessage, code_1: &str, code_2: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedErr)]
    pub fn explained_err_i32(id: LoggerMessage, code: i32);

    #[wasm_bindgen(static_method_of = Logger, js_name = explainedErr)]
    pub fn explained_err_i32_2(id: LoggerMessage, code_1: i32, code_2: i32);

    #[wasm_bindgen(static_method_of = Logger, js_name = msgHandled)]
    pub fn msg_handled() -> bool;

    #[wasm_bindgen(static_method_of = Logger, js_name = reset)]
    pub fn reset();
}
