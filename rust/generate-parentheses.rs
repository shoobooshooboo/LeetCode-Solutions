impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        Self::recurse(String::new(), n, n, &mut result);
        result
    }

    //o is remaining open parentheses, c is remaining close parentheses
    fn recurse(s: String, o: i32, c: i32, result: &mut Vec<String>){
        //if we can open more parentheses
        if o > 0{
            let mut next = s.clone();
            next.push('(');
            Self::recurse(next, o - 1, c, result);
        }
        //if we can close any parenthesis
        if o < c{
            let mut next = s.clone();
            next.push(')');
            Self::recurse(next, o, c - 1, result);
        }
        //this one is done
        else if o == 0 && c == 0{
            result.push(s);
        }
    }
}