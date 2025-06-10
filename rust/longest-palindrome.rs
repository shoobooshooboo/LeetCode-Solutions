impl Solution {
    //intuitive solution. nothing fancy here.
    pub fn longest_palindrome(s: String) -> String {
        let mut max_string = s[0..1].to_string();
        
        //scan through every substring
        for i in 0..s.len(){
            for j in (i + 1)..=s.len(){
                let substr = &s[i..j];
                //set new longest palindrome
                if Solution::is_palindrome(substr) && substr.len() > max_string.len(){
                    max_string = substr.to_string();
                }
            }
        }

        max_string
    }

    //is_palindrome function
    fn is_palindrome(s: &str) -> bool{
        for (l, r) in s.chars().zip(s.chars().rev()){
            if l != r{
                return false;
            }
        }
        true
    }
}