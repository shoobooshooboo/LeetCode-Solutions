impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        //i feel like i dont need to explain this
        match haystack.find(&needle){
            Some(n) => n as i32,
            None => -1,
        }
    }
}