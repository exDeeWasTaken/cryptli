pub mod binary {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::__rt::core::option::Option::Some;

    #[wasm_bindgen]
    pub fn binary_encrypt(str: &str) -> String {
        let mut cipher = String::new();

        let mut char_in_binary: Vec<String> = vec![];
        let mut is_first: bool = true;
        for char in str.chars() {
            if is_first {
                is_first = false;
            } else {
                cipher.push(' ');
            }
            let mut decimal_number = char as u32;

            while decimal_number > 0 {
                let modulo_result = decimal_number % 2;
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

    #[wasm_bindgen]
    pub fn binary_decrypt(str: &str) -> String {
        let mut cipher = String::new();

        for char_as_binary in str.split(' ') {
            let mut result: Option<u32> = None;
            for (index, char) in char_as_binary.chars().rev().enumerate() {
                if char == '1' {
                    let number_two: u8 = 2;
                    if let Some(mut part_result) = result {
                        part_result += number_two.pow(index as u32) as u32;
                        result = Some(part_result);
                    } else {
                        result = Some(number_two.pow(index as u32) as u32);
                    }
                }
            }
            if let Some(end_value) = result {
                if let Some(char) = std::char::from_u32(end_value) {
                    cipher.push(char);
                }
            }
        }

        cipher
    }
}