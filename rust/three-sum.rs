impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        //less intuitive two pointer solution

        //sorting helps avoid duplicates and junk
        nums.sort();
        let mut result = Vec::new();

        for i in 0..(nums.len() - 2){
            //skip duplicates
            if nums.get(i) == nums.get(i - 1){
                continue;
            }

            //these are the afformentioned two pointers
            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right{
                //get the sum
                let sum = nums[i] 
                    + nums[left] 
                    + nums[right];
                if sum == 0{
                    //update the result and pointers
                    result.push(vec!(nums[i], nums[left], nums[right]));
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1]{
                        left += 1;
                    }
                    while right > left && nums[right] == nums[right + 1]{
                        right -= 1;
                    }
                }
                else if sum > 0{
                    //too high, so dial it back a little
                    right -= 1;
                }
                else{
                    //too low, so turn things up a little
                    left += 1;
                }
            }
        }

        result
    }
}