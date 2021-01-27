pub mod binary {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn binary_encrypt(str: &str) -> String {
        let mut cipher = String::new();

        let mut char_in_binary: Vec<String> = vec![];
        for char in str.chars() {
            let mut decimal_number = char as u32;

            while decimal_number > 0 {
                let mut modulo_result = decimal_number % 2;
                char_in_binary.push(modulo_result.to_string());
                decimal_number = (decimal_number - modulo_result) / 2;
            }

            char_in_binary.reverse();

            for char in char_in_binary {
                cipher.push_str(&char[..])
            }
            char_in_binary = vec![];
        }

        cipher
    }
    //
    // #[wasm_bindgen]
    // pub fn binary_decrypt(str: &str) -> String {}
}