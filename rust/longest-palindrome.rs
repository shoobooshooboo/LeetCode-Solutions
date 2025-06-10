use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        //this time, i came up with my own, more complex algorithm
        //every palindrome that is longer than one character starts and ends with the same character
        //therefore, if we keep track of every instance of each character, we can look only at the ranges
        //that are bounded by the same letter which should greatly limit the number of substrings to check
        let mut max_string = s[0..1].to_string();
        let mut char_map: HashMap<char, Vec<usize>> = HashMap::new();
        let mut char_set: HashSet<char> = s.chars().collect();
        //make the hashmap
        for c in char_set{
            char_map.insert(c, 
            s.chars()
            .enumerate()
            .filter(|(_, ch)| c == *ch)
            .map(|(i, _)| i)
            .collect());
        }

        for (c, indeces) in char_map.iter(){
            for i in 0..indeces.len(){
                //skip substrings that are too small
                if indeces.last().unwrap() - i < max_string.len(){
                    break;
                }
                for j in ((i + 1)..indeces.len()).rev(){
                    let substr = &s[indeces[i]..=indeces[j]];
                    //skip substrings that are too small
                    if substr.len() <= max_string.len(){
                        break;
                    }
                    //update current largest substring
                    if Solution::is_palindrome(substr){
                        max_string = substr.to_string();
                    }
                }
            }

        }

        max_string
    }

    fn is_palindrome(s: &str) -> bool{
        for (l, r) in s.chars().zip(s.chars().rev()){
            if l != r{
                return false;
            }
        }
        true
    }
}