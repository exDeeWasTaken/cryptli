pub mod reverse {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn reverse(plain: String) -> String
    {
        let cipher = plain.chars().rev().collect::<String>();
        cipher
    }
}