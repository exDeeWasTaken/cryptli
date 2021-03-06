pub mod roman {
    use wasm_bindgen::prelude::*;

    //All possible roman signs
    static ROMANCODES: [(usize, &'static str); 13] = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    #[wasm_bindgen]
    pub fn roman_encrypt(plain: usize) -> String {
        let mut cypher = String::new();
        let mut plaintext = plain;

        //Check after eah iteration if plaintext still is bigger than 0
        while plaintext > 0 {

            //Go througt the romancodes and get the highest possible sign value
            for &(n, s) in &ROMANCODES {
                if n <= plaintext {
                    cypher.push_str(s);
                    plaintext -= n;
                    break;
                }
            }
        }
        cypher
    }

    #[wasm_bindgen]
    pub fn roman_decrypt(cipher: String) -> usize {

        //Create an Array that holds all the values from the signs
        let iter = cipher.chars().map(roman_to_number);

        let mut plain = 0;
        let mut prev = 0;

        //Go through the created array
        for number in iter {

            plain += number;

            //If the previous value was smaller then the current one they belong together
            if prev != 0 && number > prev {
                plain -= prev * 2;
            }

            prev = number;
        }

        plain
    }

    fn roman_to_number(roman: char) -> usize {

        let mut i = 0;
    
        //Go through the romancodes to find matching simbols and retur theyr value
        for &(value, romancode) in &ROMANCODES {
            if i % 2 == 0 && romancode.chars().nth(0) == roman.to_uppercase().next() {
                return value;
            }
            i+=1;
        }
    
        0
    }
}
