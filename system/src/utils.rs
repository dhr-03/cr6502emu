use std::ops::{Add, Sub, AddAssign, SubAssign};

pub trait Unsigned: Copy + Add + AddAssign + Sub + SubAssign {
    fn zero() -> Self;
}

impl Unsigned for u8 {
    fn zero() -> Self { 0_u8 }
}

impl Unsigned for u16 {
    fn zero() -> Self { 0_u16 }
}


//##########
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
}
