use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn sub(left: u8, right: u8) -> u8 {
    left - right
}

#[wasm_bindgen]
pub fn hello() -> u64 {
    let mut i = 0;
    loop {
        if i == 9999999999999 {
            break;
        }
        i = i + 1
    }

    i
}
