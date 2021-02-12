pub mod vigenere {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn vigenere_encrypt(plain: String, key: String) -> String
    {
        let mut cipher = String::new();

        let alphabet = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

        let plaintext: String = plain.into();

        //Passt das keyword auf die Länge des plaintextes an
        let keyword: String = key.chars().cycle().take(plaintext.len()).collect();

        // Chars im String nach und nach verschlüsseln
        plaintext.chars().zip(keyword.chars()).for_each(|(plaintext_char, keyword_char)| {

            //Hohlt den wert vom den zwei Chars
            let next_char = plaintext_char as u8 - 'A' as u8 + keyword_char as u8 - 'A' as u8;

            //Wendet modulo an um den ort des verschlüsselten charakters im alphabet string zu finden
            let next_charnumber = next_char % 26;

            //Sucht den zur Zahl zugehörigen Char
            cipher.push(alphabet.chars().nth(next_charnumber  as usize).unwrap());
        });

        cipher
    }

    #[wasm_bindgen]
    pub fn vigenere_decrypt(cipher: String, key: String) -> String
    {
        let mut plain = String::new();

        let alphabet = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

        let ciphertext: String = cipher.into();

        //Passt das keyword auf die Länge des plaintextes an
        let keyword: String = key.chars().cycle().take(ciphertext.len()).collect();

        // Wandelt den ciphertext wieder in plaintext um (Gross - Kleinschreibung nicht beachtet)
        ciphertext.chars().zip(keyword.chars()).for_each(|(ciphertext_char, keyword_char)| {

            //Hohlt den wert vom den Chars
            let keyword_number = keyword_char as u8 - 'A' as u8;
            let ciphertext_number = ciphertext_char as u8 - 'A' as u8;
            
            //Zahl zum modulorechnen (reverse vom encrypt)
            let next_char = ciphertext_number.to_string().parse::<i32>().unwrap() - keyword_number.to_string().parse::<i32>().unwrap();

            //Wendet modulo an um den ort des nichtverschlüsselten charakters im alphabet string zu finden (Workaround weil modulo normal nicht funktioniert)
            let next_charnumber = ((next_char % 26) + 26) % 26;

            //Sucht den zur Zahl zugehörigen Char
            plain.push(alphabet.chars().nth(next_charnumber  as usize).unwrap());
            });

        plain
    }
}