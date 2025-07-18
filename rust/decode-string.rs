impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut result = String::new();
        Self::decode_string_rec(&s.as_bytes(), &mut 0, &mut result);
        result
    }

    fn decode_string_rec(s: &[u8], i: &mut usize, result: &mut String){
        let mut num_start = None;
        while *i < s.len(){
            if s[*i].is_ascii_digit(){
                if num_start.is_none(){
                    num_start = Some(*i);
                }
            }
            else if s[*i] == b'['{
                let mut repeat: usize = 0;
                for j in num_start.unwrap()..*i{
                    repeat *= 10;
                    repeat += (s[j] - b'0') as usize;
                }
                println!("{repeat}");
                let str_start = result.len();
                *i += 1;
                Self::decode_string_rec(s, i, result);
                let str_end = result.len();
                result.push_str(&result[str_start..str_end].repeat(repeat - 1));
                num_start = None;
            }
            else if s[*i] == b']'{
                break;
            }
            else{
                result.push(s[*i] as char);
            }
            *i += 1;
        }
    } 
}