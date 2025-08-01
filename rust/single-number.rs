impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        //xor will essentially ignore everything that appears twice.
        //since only one thing will appear once, it'll be the only thing remaining.
        nums.into_iter().fold(0, |left, right| left ^ right)
    }
}