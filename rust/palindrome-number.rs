impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        //another very obvious answer, but slightly faster... for some reason.
        let x = x.to_string();
        for (a, b) in x.chars().zip(x.chars().rev()){
            if a != b{
                return false;
            }
        }
        true
    }
}