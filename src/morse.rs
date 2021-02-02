pub mod morse {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn morse_encrypt(str: &str) -> String {
        let mut cipher = String::new();

        let morse_alphabet = [".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..",
            ".---", "-.-", ".-..", "--", "-.", "---", "-..-", "--.-", "-.-", "...", "-", "..-", ".--",
            "-..-", "-.--", "--.."];

        for (index, char) in str.to_ascii_uppercase().chars().enumerate() {
            let array_index = (char as u32) % 65;
            if index != 0 {
                cipher.push(' ');
            }

            let morse: &str = morse_alphabet[array_index as usize];

            cipher.push_str(morse);
        }

        cipher
    }

    #[wasm_bindgen]
    pub fn morse_decrypt(str: &str) -> String {
        let mut cipher = String::new();

        let morse_alphabet = [".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..",
            ".---", "-.-", ".-..", "--", "-.", "---", "-..-", "--.-", "-.-", "...", "-", "..-", ".--",
            "-..-", "-.--", "--.."];

        for morse_char in str.split(' ') {
            let opt_index = morse_alphabet.iter().position(|&r| r == morse_char);
            if let Some(index) = opt_index {
                let char_index = (index + 97) as u8;
                cipher.push(char_index as char)
            }else{
                return "Could not decrypt morse code.".into()
            }
        }

        cipher
    }
}