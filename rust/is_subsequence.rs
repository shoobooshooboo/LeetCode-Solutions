impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        // two pointer approach. looks cleaner but is less versatile than
        // the iterator approach i used previously.
        let s = s.as_bytes();
        let t = t.as_bytes();
        let (mut i, mut j) = (0, 0);
        while j < t.len(){
            if s[i] == t[j]{
                i += 1;
            }
            if i > s.len(){
                break;
            }

            j += 1;
        }

        i == s.len()
    }
}