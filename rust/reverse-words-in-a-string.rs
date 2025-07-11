impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        s = s.trim().to_string();
        while s.contains("  ") {s = s.replace("  ", " ");}
        let (mut i, mut j) = (0, s.len() - 1);
        let (mut left_start, mut left_end, mut right_start, mut right_end) = (0,0,0,0);
        while i < j{
            //get left word
            while s.get(i..(i + 1)).is_some_and(|c| c == " "){
                i += 1;
            }
            left_start = i;
            while s.get(i..(i + 1)).is_some_and(|c| c != " "){
                i += 1;
            }
            left_end = i;

            //get right word
            while s.get(j..(j + 1)).is_some_and(|c| c == " "){
                j -= 1;
            }
            right_end = j + 1;
            while s.get(j..(j + 1)).is_some_and(|c| c != " "){
                j -= 1;
            }
            right_start = j + 1;


            if j > s.len(){
                break;
            }
            //swap
            let temp = s[right_start..right_end].to_string();
            s.replace_range(right_start..right_end, &s[left_start..left_end].to_string());
            s.replace_range(left_start..left_end, &temp);
            //update i and j
            j = right_end + (left_start - left_end) - 1;
            i = left_start + (right_end - right_start) + 1;
        }

        s
    }
}