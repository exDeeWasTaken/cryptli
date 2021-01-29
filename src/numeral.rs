pub mod numeral {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn numeral_encode(str: &str, radix: u32){
        let mut cipher = String::new();
        for char in str.chars() {
            let index = char as u32;

            cipher.push(index.read)
        }
    }
}