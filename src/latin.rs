pub mod latin {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn latin_encrypt(str: &str) -> String {
        let mut cipher = String::new();

        for (index, char) in str.chars().enumerate() {
            if index != 0 {
                cipher.push(' ')
            }

            if char.is_ascii_alphabetic() {
                let ascii_index = char as u32;
                let num_index: u32;
                if char.is_ascii_lowercase() {
                    num_index = ascii_index - 96;
                } else {
                    num_index = ascii_index - 64;
                }
                cipher.push_str(&*format!("{:?}", num_index));
            } else {
                cipher.push(char);
            }
        }

        cipher
    }

    #[wasm_bindgen]
    pub fn latin_decrypt() {}
}