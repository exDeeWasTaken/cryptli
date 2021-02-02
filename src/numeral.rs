pub mod numeral {
    use wasm_bindgen::prelude::*;
    use std::char;

    #[wasm_bindgen]
    pub fn numeral_encrypt(str: &str, radix: u32) -> String {
        let nums =
            ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

        let mut cipher = String::new();
        if radix > nums.len() as u32 {
            return "Error: Radix bigger than 36".into();
        }

        if radix < 2 {
            return "Error: Radix smaller than 2".into();
        }

        let mut char_in_other_radix: Vec<String> = vec![];
        for (index, char) in str.chars().enumerate() {
            if index > 0 {
                cipher.push(' ');
            }

            let mut char_num = char as u32;

            while char_num > 0 {
                let current_num = char_num % radix;
                char_in_other_radix.push(nums[current_num as usize].to_string());
                char_num = (char_num - current_num) / radix;
            }

            char_in_other_radix.reverse();

            for char in char_in_other_radix {
                cipher.push_str(&char[..])
            }

            char_in_other_radix = vec![];
        }

        cipher
    }

    #[wasm_bindgen]
    pub fn numeral_decrypt(str: &str, from_radix: u32) -> String {
        let nums =
            ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

        let mut cipher = String::new();

        if from_radix > nums.len() as u32 {
            return "Error: Radix bigger than 36".into();
        }

        if from_radix < 2 {
            return "Error: Radix smaller than 2".into();
        }

        let numbers_from_string = str.split(' ');

        for number_string in numbers_from_string {
            let mut value: u32 = 0;
            for (index, charix) in number_string.chars().rev().enumerate() {
                let char_index_in_nums = nums.iter().position(|&r| r == charix);
                if let Some(array_index) = char_index_in_nums {
                    value += array_index as u32 * from_radix.pow(index as u32);
                } else {
                    return "Char not found in possible numbers array".into();
                }
            }
            let new_char = char::from_u32(value);

            if let Some(chari) = new_char {
                cipher.push(chari)
            }else{
                return "Something went wrong".into();
            }
        }

        cipher
    }
}