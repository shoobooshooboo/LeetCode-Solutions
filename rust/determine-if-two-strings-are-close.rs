impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        //theres at most 26 elements, so instead of a hashmap, just use an array.
        let mut freq1 = [0;26];
        word1.as_bytes().into_iter().for_each(|c| { freq1[(c - b'a') as usize] += 1;});
        let mut freq2 = [0;26];
        word2.as_bytes().into_iter().for_each(|c| { freq2[(c - b'a') as usize] += 1;});

        //make sure all the same characters are used
        for i in 0..26{
            if (freq1[i] > 0) != (freq2[i] > 0){
                return false;
            }
        }

        //make sure all the same frequencies appear
        freq1.sort();
        freq2.sort();
        freq1 == freq2
    }
}