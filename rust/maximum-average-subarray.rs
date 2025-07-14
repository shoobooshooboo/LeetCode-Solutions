impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        //sliding window solution.
        let k = k as usize;
        //get first sum
        let mut sum = 0;
        for i in 0..k{
            sum += nums[i];
        }
        let mut max = sum;
        for i in k..nums.len(){
            //now to get each new sum, you just add the new number and subtract the last one.
            sum += nums[i] - nums[i - k];
            if sum > max{
                max = sum;
            }
        }

        //turn the sum into the average.
        max as f64 / k as f64
    }
}