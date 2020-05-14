use wasm_bindgen::prelude::wasm_bindgen;

// these expressions always return (if anything was matched) a fixed amount of captures,
// optional capture groups return ""


#[wasm_bindgen(module = "/js_snippets/regex.js")]
extern {
    /*
    normal modes, returns:
       #?,
       <-- COMMON -->
       b/$//lo /hi /%,
       value/identifier
       </---->
    */
    #[wasm_bindgen(js_name = "reNormalAddressing")]
    pub fn js_re_nrm(line: &str, bounds: &mut [usize]);



    /*
    indexed, returns:
           (/&/\*?,
           <-- COMMON -->
           b/$//lo /hi /%,
           value/identifier
           </---->
           )?,
           (X/Y)?,
           )?
    */
    #[wasm_bindgen(js_name = "reIndexedAddressing")]
    pub fn js_re_inx(line: &str, bounds: &mut [usize]);
}

