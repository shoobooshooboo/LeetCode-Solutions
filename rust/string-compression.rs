impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        //pretty straightforward solution
        let mut i = 0;
        while i < chars.len(){
            //get the current char
            let c = chars[i];
            i += 1;
            //where the number will start
            let start = i;
            
            //see how many of that character there are in a row.
            let mut count = 1;
            while chars.get(i).is_some_and(|x| *x == c){
                count += 1;
                i += 1;
            }
            
            if count == 1{
                continue;
            }

            //get the num as a vector of the characters and cram them into the chars Vec
            let num: Vec<char> = count.to_string().chars().collect();
            let mut j = start;
            for digit in num{
                chars[j] = digit;
                j += 1;
            }
            
            //then remove the extra letters
            while j < i{
                chars.remove(j);
                i -= 1;
            }
        }

        chars.len() as i32
    }
}