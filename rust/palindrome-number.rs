impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        //very obvious, easy solution.
        let x = x.to_string();
        let rev: String = x.chars().rev().collect();

        x == rev
    }
}