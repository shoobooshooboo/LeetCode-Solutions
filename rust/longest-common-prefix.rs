use std::str::Chars;
use std::cmp::min;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        //change strings to byte arrays for easy indexing
        let strs: Vec<_> = strs.into_iter().map(|s| s.into_bytes()).collect();
        let mut prefix = String::new();

        //minimum string length
        let mut len = strs.iter().fold(strs[0].len(), |a, s| min(a, s.len()));
        'outer: for i in 0..len{
            for j in 0..strs.len(){
                //if this char doesnt match up, stop looking
                if strs[j][i] != strs[0][i]{
                    break 'outer;
                }
            }
            //all strings have this char, so add it to the prefix
            prefix.push(strs[0][i] as char);
        }

        prefix
    }
}