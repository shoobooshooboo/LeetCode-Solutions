impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars(){
            if c == ')' && stack.last() == Some(&'('){
                stack.pop();
            }
            else if c == ']' && stack.last() == Some(&'['){
                stack.pop();
            }
            else if c == '}' && stack.last() == Some(&'{'){
                stack.pop();
            }
            else if c == '(' || c == '[' || c == '{'{
                stack.push(c);
            }
            else{
                return false;
            }
        }

        stack.is_empty()
    }
}