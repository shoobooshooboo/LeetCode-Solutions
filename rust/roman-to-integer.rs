use std::collections::HashMap;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        //byte array for easy indexing
        let s = s.into_bytes();
        let numerals = HashMap::from([
            (b'I', 1),
            (b'V', 5),
            (b'X', 10),
            (b'L', 50),
            (b'C', 100),
            (b'D', 500),
            (b'M', 1000),
        ]);

        let mut result = 0;
        let mut i = 0;
        while i < s.len(){
            //when the next numeral is greater than the current one, subtract
            if i + 1 < s.len() && numerals[&s[i]] < numerals[&s[i + 1]]{
                result += numerals[&s[i + 1]] - numerals[&s[i]];
                i += 2;
            }else{
            //otherwise, add
                result += numerals[&s[i]];
                i += 1;
            }
        }

        result
    }
}