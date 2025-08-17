impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        //you will never skip more than 2 houses in a row
        //because you would just be losing money. so it's
        //very similar to the staircase problem.

        //updates each house value with the max value between the one two behind and the one three behind.
        for i in 2..nums.len(){
            nums[i] += *nums.get(i - 2).max(nums.get(i - 3)).unwrap();
        }

        //return the max of the last two
        *nums.last().max(nums.get(nums.len() - 2)).unwrap()
    }
}