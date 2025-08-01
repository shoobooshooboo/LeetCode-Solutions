impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        //very simple dynamic programming solution

        //n is going to be positive or 0 so making it usize
        //simplifies things.
        let n = n as usize;
        let mut result = Vec::with_capacity(n + 1);
        result.resize(n + 1, 0);

        for mut i in 1..=n{
            //if i is even, the bit count is equal to that of i >> 1.
            //if it's odd, it's 1 + i>>1's bit count
            result[i] = 
                if i & 1 == 1 {1} else {0} 
                + result[i >> 1];
        }

        result
    }
}