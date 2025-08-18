impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        //2d dp array
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = 1;

        for i in 0..m{
            for j in 0..n{
                //number of unique paths is just the sum of the ones above and to the left
                dp[i][j] += *dp[i].get(j - 1).unwrap_or(&0) 
                    + *dp.get(i - 1).unwrap_or(&vec![0]).get(j).unwrap_or(&0);
            }
        }
        *dp.last().unwrap().last().unwrap()
    }
}