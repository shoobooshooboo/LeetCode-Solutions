impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        //prefix/postfix sum solution.
        let mut postfix: i32 = nums.iter().sum();
        let mut prefix: i32 = 0;
        for (i, n) in nums.iter().enumerate(){
            //update postfix first.
            postfix -= n;
            //check if they're the same
            if prefix == postfix{
                return i as i32;
            }
            //now update prefix
            prefix += n;
        }
        -1
    }
}