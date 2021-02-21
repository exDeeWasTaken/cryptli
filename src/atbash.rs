pub mod atbash {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn atbash_encrypt(plain: String) -> String
    {
        let mut cipher = String::new();

        let alphabet = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

        let plaintext: String = plain.into();

        // Encrypt char by char
        plaintext.chars().for_each(|plaintext_char| {

            //u8 value of the two chars
            let next_char = plaintext_char as u8 - 'A' as u8;

            //Get next charnumber
            let next_charnumber = 25 - next_char;

            //Searches to next_number correspnding char
            cipher.push(alphabet.chars().nth(next_charnumber  as usize).unwrap());
        });

        cipher
    }

    #[wasm_bindgen]
    pub fn atbash_decrypt(cipher: String) -> String
    {
        let mut plain = String::new();

        let alphabet = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

        let ciphertext: String = cipher.into();

         // Encrypt char by char
         ciphertext.chars().for_each(|ciphertext_char| {

            //u8 value of the two chars
            let next_char = ciphertext_char as u8 - 'A' as u8;

            //Get next charnumber
            let next_charnumber = 25 - next_char;

            //Searches to next_number correspnding char
            plain.push(alphabet.chars().nth(next_charnumber  as usize).unwrap());
        });

        plain
    }
}