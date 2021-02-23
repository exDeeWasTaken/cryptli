pub mod scytale {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn scytale_encrypt(plain: String, height: usize) -> String
    {
        let mut cipher = String::new();

        let plaintext_vec: Vec<char> = plain.to_string().chars().collect();

        //counter for number of wraps already done
        let mut c = 0;

        //index of current wrapposition
        let mut i = 0;

        // Encrypt char by char
        while cipher.len() < plain.len() {
            if (i + c) > plain.len() - 1 { i = 0; c += 1; }

            cipher.push(plaintext_vec[i + c]);

            i = i + height;
        }

        cipher
    }

    #[wasm_bindgen]
    pub fn scytale_decrypt(cipher: String, height: usize) -> String
    {
        let mut plain = String::new();

        let cipher_vec: Vec<char> = cipher.to_string().chars().collect();

        let ofset: usize;

        if cipher.len() % height == 0 { ofset = cipher.len() / height; }
        else { ofset = cipher.len() / height + 1; }
        
        //counter for number of wraps already done
        let mut c = 0;

        //index of current wrapposition
        let mut i = 0;

        // Decrypt char by char
        while plain.len() < cipher.len() {
            if (i + c) > cipher.len() - 1 { i = 0; c += 1;  }

            plain.push(cipher_vec[i + c]);

            i = i + ofset;
        }

        plain
    }
}