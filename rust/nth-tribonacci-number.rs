impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        //about as simple as dynamic programming solutions can get.

        //handle edge cases
        if n == 2{
            return 1;
        }else if n < 2{
            return n;
        }

        //n being as usize is just kinda nice
        let n = n as usize;
        //dp array (i dont have much experience with dp yet so idk what else to call this variable)
        let mut dp = Vec::with_capacity(n + 1);
        //load up initial values
        dp.push(0);
        dp.push(1);
        dp.push(1);
        //calculate procedurally
        for i in 3..=n{
            dp.push(dp[i - 1] + dp[i - 2] + dp[i - 3]);
        }

        //the nature of dp means that the last one will be the answer
        dp.pop().unwrap()
    }
}