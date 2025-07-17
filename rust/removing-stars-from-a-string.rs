impl Solution {
    pub fn remove_stars(mut s: String) -> String {
        //basically just using the result string like a stack
        let mut result = String::new();

        for c in s.chars(){
            if c == '*'{
                result.pop();
            }
            else{
                result.push(c);
            }
        }

        result
    }
}