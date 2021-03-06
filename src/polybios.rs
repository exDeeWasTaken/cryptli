pub mod polybios {
    use wasm_bindgen::prelude::*;

    static POLYBIOSSQUARE: [[char; 6]; 6] = [
        ['A', 'B', 'C', 'D', 'E', 'F'],
        ['G', 'H', 'I', 'J', 'K', 'L'],
        ['M', 'N', 'O', 'P', 'Q', 'R'],
        ['S', 'T', 'U', 'V', 'W', 'X'],
        ['Y', 'Z', '0', '1', '2', '3'],
        ['4', '5', '6', '7', '8', '9'],
    ];

    #[wasm_bindgen]
    pub fn polybios_encrypt(plain: String, key: String) -> String {
        let polybiossquare = create_polybiossquare(key);

        let mut cypher = String::new();

        let plaintext: String = plain.into();

        for c in plaintext.to_ascii_uppercase().chars() {
            
            let mut next_number = 0;

            for (i, x) in polybiossquare[0].iter().enumerate() {
                if c == *x { next_number = i + 11; }
            }
            for (i, x) in polybiossquare[1].iter().enumerate() {
                if c == *x { next_number = i + 21; }
            }
            for (i, x) in polybiossquare[2].iter().enumerate() {
                if c == *x { next_number = i + 31; }
            }
            for (i, x) in polybiossquare[3].iter().enumerate() {
                if c == *x { next_number = i + 41; }
            }
            for (i, x) in polybiossquare[4].iter().enumerate() {
                if c == *x { next_number = i + 51; }
            }
            for (i, x) in polybiossquare[5].iter().enumerate() {
                if c == *x { next_number = i + 61; }
            }

            cypher.push(next_number.to_string().chars().nth(0).unwrap());
            cypher.push(next_number.to_string().chars().nth(1).unwrap());
            cypher.push(' ');
        }    

        cypher.truncate(cypher.len() - 1);
        cypher
    }

    #[wasm_bindgen]
    pub fn polybios_decrypt(cypher: String, key: String) -> String {
        let polybiossquare = create_polybiossquare(key);

        let mut plain = String::new();

        let cyphertext: String = cypher.into();

        plain
    }

    //Functions for encrypt and decrypt

    fn create_polybiossquare(key: String) -> [[char; 6]; 6] {
        let mut polybiossquare = POLYBIOSSQUARE;

        let mut usedchars = String::new();
        let mut doubles = 0;

        for (i, c) in key.to_ascii_uppercase().chars().enumerate() {
            if !check_for_double(c, usedchars.to_string()) {
                usedchars.push(c);
                let matcher = i - doubles;
                match matcher {
                    0..=5 => polybiossquare[0][matcher] = c,
                    6..=11 => polybiossquare[1][matcher - 6] = c,
                    12..=17 => polybiossquare[2][matcher - 12] = c,
                    18..=23 => polybiossquare[3][matcher - 18] = c,
                    24..=29 => polybiossquare[4][matcher - 24] = c,
                    30..=35 => polybiossquare[5][matcher - 30] = c,
                    _ => println!("Error"),
                }
            } else {
                doubles += doubles;
            }
        }

        polybiossquare = fix_polybiossquare(usedchars, polybiossquare);

        polybiossquare
    }
    fn check_for_double(ch: char, checkchars: String) -> bool {
        for c in checkchars.chars() {
            if ch == c {
                return true;
            }
        }

        false
    }

    fn fix_polybiossquare(usedchars: String, old_polybiossquare: [[char; 6]; 6]) -> [[char; 6]; 6] {
        let mut polybiossquare = old_polybiossquare;

        let mut allchars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut placeholder;
        for c in usedchars.chars() {
            placeholder = allchars.replace(c, "");
            allchars = &*placeholder;
        }

        for (i, c) in allchars.chars().enumerate() {
            let matcher = i + usedchars.len();

            match matcher {
                0..=5 => polybiossquare[0][matcher] = c,
                6..=11 => polybiossquare[1][matcher - 6] = c,
                12..=17 => polybiossquare[2][matcher - 12] = c,
                18..=23 => polybiossquare[3][matcher - 18] = c,
                24..=29 => polybiossquare[4][matcher - 24] = c,
                30..=35 => polybiossquare[5][matcher - 30] = c,
                _ => println!("Error"),
            }
        }

        polybiossquare
    }
}
