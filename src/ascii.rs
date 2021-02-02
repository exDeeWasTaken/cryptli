pub mod ascii {
    use wasm_bindgen::prelude::*;
    use std::str::FromStr;
    use std::char;

    #[wasm_bindgen]
    pub fn ascii_encrypt(str: &str) -> String {
        let mut cipher = String::new();
        for (index, char) in str.chars().enumerate() {
            if index != 0 {
                cipher.push(' ');
            }
            cipher.push_str(&*(char as u32).to_string())
        }

        cipher
    }

    #[wasm_bindgen]
    pub fn ascii_decrypt(str: &str) -> String {
        let mut cipher = String::new();

        for number_string in str.split(' ') {
            let num = u32::from_str(number_string);
            match num {
                Ok(v) => {
                    if let Some(chari) = char::from_u32(v) {
                        cipher.push(chari);
                    } else {
                        return "Failed to get ascii character".into();
                    }
                }
                Err(_) => {
                    return "Failed to get ascii character".into();
                }
            }
        }

        cipher
    }
}