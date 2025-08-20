impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0;text2.len()];text1.len()];
        let (text1, text2) = (text1.into_bytes(), text2.into_bytes());

        for i in 0..text1.len(){
            for j in 0..text2.len(){
                if text1[i] == text2[j]{
                    dp[i][j] = *dp.get(i - 1).unwrap_or(&Vec::new()).get(j - 1).unwrap_or(&0) + 1;
                }else{
                    dp[i][j] = *dp.get(i).unwrap_or(&Vec::new()).get(j - 1).unwrap_or(&0)
                    .max(dp.get(i - 1).unwrap_or(&Vec::new()).get(j).unwrap_or(&0));
                }

            }
        }
        dp.pop().unwrap().pop().unwrap()
    }
}