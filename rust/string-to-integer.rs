impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        //trim leading whitespace
        let mut s = s.trim_start();
        let mut result: i32 = 0;
        //handles a leading character of - or +
        let is_negative = 
            if s.starts_with("-") {s = &s[1..]; true}
            else if s.starts_with("+"){s = &s[1..]; false}
            else {false};

        for c in s.chars(){
            if !c.is_numeric(){
                break;
            }
            //multiplication and addition are dangerous, so just be safe.
            result = match result.checked_mul(10){
                Some(n) => n,
                None => return if is_negative{i32::MIN} else {i32::MAX},
            };
            result = match result.checked_add(c.to_digit(10).unwrap() as i32){
                Some(n) => n,
                None => return if is_negative{i32::MIN} else {i32::MAX},
            };
        }

        if is_negative {-result} else {result}
    }
}