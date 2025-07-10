impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let mut i = 0;
        let len = std::cmp::max(word1.len(), word2.len());
        while i < len{
            match word1.get(i..(i + 1)){
                Some(s) => result.push_str(s),
                None => (),
            }
            match word2.get(i..(i + 1)){
                Some(s) => result.push_str(s),
                None => (),
            }
            i += 1;
        }
        result
    }
}