impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        //s to bytes and k to usize for easier indexing
        let s = s.as_bytes();
        let k = k as usize;
        //get first vowel count
        let mut vowel_count = 0;
        for i in 0..k{
            if Self::is_vowel(s[i]){
                vowel_count += 1;
            }
        }
        let mut max = vowel_count;
        //sliding window now
        for i in k..s.len(){
            if Self::is_vowel(s[i - k]){
                vowel_count -= 1;
            }
            if Self::is_vowel(s[i]){
                vowel_count += 1;
            }
            if vowel_count > max{
                max = vowel_count;
            }
        }

        max
    }

    fn is_vowel(c: u8) -> bool{
        match c.to_ascii_lowercase(){
            b'a' | b'e' | b'i' | b'o' | b'u' => true,
            _ => false
        }
    }
}