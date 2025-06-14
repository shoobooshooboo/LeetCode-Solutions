impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        //literally the same as 3sum but just keeping track of the closest instead of
        //looking for exact matches
        let mut closest = nums[0] + nums[1] + nums[2];
        nums.sort();

        for i in 0..(nums.len() - 2){
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            while j < k{
                let sum = nums[i] + nums[j] + nums[k];
                if sum == target{
                    return sum;
                }

                if (target - sum).abs() < (target - closest).abs(){
                    closest = sum;
                }
                else if sum > target{
                    k -= 1;
                }
                else{
                    j += 1;
                }
            }
        }

        closest
    }
}