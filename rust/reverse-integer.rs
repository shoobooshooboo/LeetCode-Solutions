impl Solution {
    pub fn reverse(x: i32) -> i32 {
        //handle edge cases
        if x == 0 || x == i32::MIN {return 0;}

        let is_negative = x < 0;

        //store digits
        let mut x = x.abs();
        let mut digits = Vec::new();
        while x > 0{
            digits.push(x % 10);
            x /= 10;
        }

        //put digits back into x
        for i in 0..(digits.len() - 1){
            x += digits[i];
            //multiplication may fail
            x = match x.checked_mul(10){
                Some(n) => n,
                None => return 0,
            }
        }

        //final addition might fail (maybe???). just being safe.
        match x.checked_add(digits.pop().unwrap()){
            Some(n) => if is_negative {-n} else {n},
            None => 0,
        }
    }
}