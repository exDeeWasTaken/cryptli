pub mod cesar {
    use wasm_bindgen::prelude::*;
    use std::char;

    #[wasm_bindgen]
    pub fn cesar(str: &str, mut shift_amount: i32) -> String {
        let mut cipher = String::new();
        for char in str.chars() {
            if char.is_ascii_alphabetic() {
                let index = char as u32;

                let mut min_val: i32 = 97;

                if index >= 65 && index <= 90 {
                    min_val = 65;
                }

                if shift_amount < 0 {
                    shift_amount = 26 - ((shift_amount * -1) % 26);
                }

                let new_index = ((index as i32 + shift_amount - min_val) % 26 + min_val) as u8;

                cipher.push(new_index as char);
            } else {
                cipher.push(char);
            }
        }
        cipher
    }
}