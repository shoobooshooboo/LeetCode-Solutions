impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        //two iterator solution. basically two pointer but maybe a little dirtier.
        //really just did this cause i wanted to use a for loop.
        let mut subsequence = s.chars();
        let mut sub_c = subsequence.next();
        for c in t.chars(){
            match &sub_c{
                Some(sc) => {
                    if *sc == c{
                        sub_c = subsequence.next();
                    }
                },
                None => break,
            }
        }
        sub_c.is_none()
    }
}