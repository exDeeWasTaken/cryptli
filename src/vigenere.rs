pub mod vigenere {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn vigenere_encrypt(plain: String, key: String) -> String
    {
        let mut cipher = String::new();

        let alphabet = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

        let plaintext: String = plain.into();

        //fits keyword to lengh of plaintext
        let keyword: String = key.chars().cycle().take(plaintext.len()).collect();

        // Encrypt char by char
        plaintext.chars().zip(keyword.chars()).for_each(|(plaintext_char, keyword_char)| {

            //u8 value of the two chars
            let next_char = plaintext_char as u8 - 'A' as u8 + keyword_char as u8 - 'A' as u8;

            //Use modulo to get next charvalue
            let next_charnumber = next_char % 26;

            //Searches to next_number correspnding char
            cipher.push(alphabet.chars().nth(next_charnumber as usize).unwrap());
        });

        cipher
    }

    #[wasm_bindgen]
    pub fn vigenere_decrypt(cipher: String, key: String) -> String
    {
        let mut plain = String::new();

        let alphabet = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

        let ciphertext: String = cipher.into();

        //fits keyword to lengh of plaintext
        let keyword: String = key.chars().cycle().take(ciphertext.len()).collect();

        // Decrypts ciphertext to plaintext (only lowercase)
        ciphertext.chars().zip(keyword.chars()).for_each(|(ciphertext_char, keyword_char)| {

            //Gets value of chars
            let keyword_number = keyword_char as u8 - 'A' as u8;
            let ciphertext_number = ciphertext_char as u8 - 'A' as u8;

            //get numbervalue of chars (reverse vrom encrypt)
            let next_char = ciphertext_number.to_string().parse::<i32>().unwrap() - keyword_number.to_string().parse::<i32>().unwrap();

            //Use modulo to get the location of notencrypted char (Wokaround modulo function because normal modulo doesn't work)
            let next_charnumber = ((next_char % 26) + 26) % 26;

            //Searches to next_charnumber correspnding char
            plain.push(alphabet.chars().nth(next_charnumber as usize).unwrap());
        });

        plain
    }
}