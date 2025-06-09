use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let input_len = s.len();
        let mut max_length = 0;
        for i in 0..input_len{
            //early break of current max_length if there can't be a longer substring
            if max_length + i > input_len{
                break;
            }

            let mut length = 0; 
            //hashset to keep track of unique characters
            let mut chars = HashSet::new();
            for c in s[i..].chars(){
                //count unique characters and break on a duplicate
                if !chars.insert(c){
                    break;
                }
                length += 1;
            }
            //set the new max_length
            max_length = std::cmp::max(max_length, length);
        }

        max_length as i32
    }
}