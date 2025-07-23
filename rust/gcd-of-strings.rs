impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        //get the length of the prefix
        let mut prefix_len = 0;
        for (a, b) in str1.chars().zip(str2.chars()){
            if a != b {
                break;
            }
            prefix_len += 1;
        }

        //go through every prefix from greatest to least
        for gcd_len in (1..=prefix_len).rev(){
            //ignore prefixes that cant divide both strings 
            if str1.len() % gcd_len != 0 || str2.len() % gcd_len != 0{
                continue;
            }
            let prefix = str1.get(..gcd_len).unwrap();
            println!("{prefix}");

            //return a prefix if it divides both strings.
            if str1.chars().zip(prefix.chars().cycle()).all(|(a, b)| a == b)
                && str2.chars().zip(prefix.chars().cycle()).all(|(a, b)| a == b){
                    return prefix.to_string();
            }
        }

        "".into()
    }
}