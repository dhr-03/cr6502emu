use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/js_snippets/logger.js")]
extern {
    pub type Logger;

    #[wasm_bindgen(static_method_of = Logger, js_name = setup)]
    pub fn setup(selector: &str);

    #[wasm_bindgen(static_method_of = Logger, js_name = setCurrentLine)]
    pub fn set_current_line(line_num: i32);

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
}
