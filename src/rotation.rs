pub mod rotation {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn rotation_encrypt(plain: String, key: usize) -> String
    {
        if key > 26 {
            return "Only numbers between 0 and 26 allowed".to_string();
        }else if key == 0 {
            return plain;
        }

        let mut cipher = String::new();

        let plaintext_vec: Vec<char> = plain.to_string().chars().collect();

        let alphabet = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

        //index of current position
        let mut i = 0;

        // Encrypt char by char
        while cipher.len() < plain.len() {

            let next_char = plaintext_vec[i];

            //find next index of Char
            let next_index_in_alph = alphabet.iter().position(|x| *x == next_char).unwrap();

            let mut next_char_num = next_index_in_alph + key;

            //Handle start from A case
            if next_char_num > 25 { next_char_num = next_char_num - 26 }

            cipher.push(alphabet[next_char_num]);

            i += 1;
        }

        cipher
    }

    #[wasm_bindgen]
    pub fn rotation_decrypt(cipher: String, key: usize) -> String
    {
        if key > 26 {
            return "Only numbers between 0 and 26 allowed".to_string();
        }else if key == 0 {
            return cipher;
        }

        let mut plain = String::new();

        let cipher_vec: Vec<char> = cipher.to_string().chars().collect();

        let alphabet = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

        //index of current position
        let mut i = 0;

        // Encrypt char by char
        while plain.len() < cipher.len() {

            let next_char = cipher_vec[i];

            //find next index of Char
            let next_index_in_alph = alphabet.iter().position(|x| *x == next_char).unwrap();

            let mut next_char_num = next_index_in_alph + key;

            //Handle start from A case
            if next_char_num > 25 { next_char_num = next_char_num - 26 }

            plain.push(alphabet[next_char_num]);

            i += 1;
        }

        plain
    }
}