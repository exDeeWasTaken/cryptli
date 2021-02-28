pub mod reverse {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn reverse_encrypt(plain: String) -> String
    {
        let cipher = plain.chars().rev().collect::<String>();
        cipher
    }

    #[wasm_bindgen]
    pub fn reverse_decrypt(cipher: String) -> String
    {
        let plain = cipher.chars().rev().collect::<String>();

        plain
    }
}