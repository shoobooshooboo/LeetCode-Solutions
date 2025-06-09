use std::collections::HashSet;
use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let input_len = s.len();
        //left and right boundary
        let (mut i, mut j) = (0, 0);
        let mut max_length = 0;
        //for tracking unique chars
        let mut char_set = HashSet::new();
        let chars: Vec<char> = s.chars().collect();
        while j < input_len{
            //check if new character is a duplicate
            if char_set.insert(&chars[j]){
                //if it is, move right the boundary
                j += 1;
            }else{
                //otherwise, remove the first character
                char_set.remove(&chars[i]);
                //and adjust both bounds as needed
                if i == j{
                    j += 1;
                }
                i += 1;
            }
            //update the max length
            max_length = max(max_length, j - i);
        }

        max_length as i32
    }
}