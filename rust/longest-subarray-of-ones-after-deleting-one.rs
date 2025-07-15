impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        //the same as max consecutive 1s III but with k = 1.

        //handle edge case
        if !nums.contains(&0){
            return nums.len() as i32 - 1;
        }
        //inchworm window
        let (mut i, mut j) = (0, 0);
        let mut max = 0;
        loop{
            //scoot the head forward until it finds a 1.
            while let Some(n) = nums.get(j){
                if *n == 0{
                    break;
                }
                j += 1;
            }

            //now get every 1 that follows
            j += 1;
            while let Some(n) = nums.get(j){
                if *n != 1{
                    break;
                }
                j += 1;
            }

            //store the max length so far
            max = max.max(j - i);
            if j >= nums.len(){
                break;
            }

            //now bring the tail forward
            while nums.get(i).is_some_and(|n| *n == 1){
                i += 1;
            }
            i += 1;
        }
        max as i32 - 1
    }
}