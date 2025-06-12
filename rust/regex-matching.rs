impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        //change strings to byte arrays
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (n, m) = (s.len(), p.len());
        //dynamic programming table
        let mut dp = vec![vec![false; m + 1]; n + 1];
        //when both strings are empty
        dp[0][0] = true;

        //honestly, i only kinda understand the rest of this
        //but not enough to be able to explain it well.
        //this is my introduction to dynamic programming 
        //so i used a guide to help me out.
        for i in 1..=m{
            if p[i - 1] == b'*'{
                dp[0][i] = dp[0][i - 2];
            }
        }

        for i in 1..=n{
            for j in 1..=m{
                if p[j - 1] == b'.' || p[j - 1] == s[i - 1]{
                    dp[i][j] = dp[i - 1][j - 1];
                }else if p[j - 1] == b'*'{
                    dp[i][j] = dp[i][j - 2] || (dp[i - 1][j] && (s[i - 1] == p[j - 2] || p[j - 2] == b'.'))
                }
            }
        }
        dp[n][m]
    }
}