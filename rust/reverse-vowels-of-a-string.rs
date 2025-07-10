impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        //simpler solution. only one loop.

        //s to bytes for easy indexing.
        let mut s = s.into_bytes();

        //two pointer approach.
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j{
            //while s[i] is not a vowel, increment i.
            if !Self::is_vowel(&s[i]){
                i += 1;
            } 
            //while s[j] is not a vowel, decrement j.
            else if !Self::is_vowel(&s[j]){
                j -= 1;
            }
            //if both are vowels, swap and then update i and j.
            else{
                s.swap(i, j);
                i += 1;
                j -= 1;
            }
        }

        String::from_utf8(s).unwrap()
    }

    fn is_vowel(c: &u8) -> bool{
        match c.to_ascii_lowercase(){
            b'a' | b'e' | b'i' | b'o' | b'u' => true,
            _ => false,
        }
    }
}