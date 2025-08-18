impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        //dynamic programming solution
        const modulo: i128 = 10i128.pow(9) + 7i128;
        let n = n as usize;
        //edge cases
        if n <= 3{
            let initial = [1, 1, 2, 5];
            return initial[n];
        }
        //dp array with default values
        let mut dp: Vec<i128> = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 5;
        //now just populate all the new ones
        for i in 4..=n{
            dp[i] = (2 * dp[i - 1] + dp[i - 3]) % modulo;
        }

        *dp.last().unwrap() as i32
    }
}