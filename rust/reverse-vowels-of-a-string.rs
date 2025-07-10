impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        //change string to bytes for easy indexing.
        let mut s = s.into_bytes();
        //vector of vowel indeces
        let mut vowel_indeces = Vec::new();
        //find all vowels
        for (i, c) in s.iter().enumerate(){
            if Self::is_vowel(c){
                vowel_indeces.push(i);
            }
        }

        //no vowels. return.
        if vowel_indeces.len() == 0{
            return String::from_utf8(s).unwrap();
        }

        //two pointers swapping all vowels
        let (mut i, mut j) = (0, vowel_indeces.len() - 1);
        while i < j{
            s.swap(vowel_indeces[i], vowel_indeces[j]);
            i += 1;
            j -= 1;
        }

        //back to string
        String::from_utf8(s).unwrap()
    }

    fn is_vowel(c: &u8) -> bool{
        match c.to_ascii_lowercase(){
            b'a' | b'e' | b'i' | b'o' | b'u' => true,
            _ => false,
        }
    }
}