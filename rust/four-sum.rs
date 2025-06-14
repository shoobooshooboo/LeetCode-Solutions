use std::collections::HashSet;
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        //handling edge case
        if nums.len() < 4{
            return Vec::new();
        }
        //basically just throwing an extra loop around 3sum
        //using a hash set to avoid duplicates
        let mut result = HashSet::new();
        nums.sort();

        for i in 0..(nums.len() - 3){
            for j in (i + 1)..(nums.len() - 2){
                if j > i + 1 && nums[j] == nums[j - 1]{continue;}
                let (mut l, mut r) = (j + 1, nums.len() - 1);

                while l < r{
                    let mut overflow = false;
                    let mut sum = nums[i];
                    //use checked_add to manage overflow
                    sum = match sum.checked_add(nums[j]){
                        Some(n) => n,
                        None => {overflow = true; 0},
                    };
                    sum = match sum.checked_add(nums[l]){
                        Some(n) => n,
                        None => {overflow = true; 0},
                    };
                    sum = match sum.checked_add(nums[r]){
                        Some(n) => n,
                        None => {overflow = true; 0},
                    };

                    if !overflow && sum == target{
                        result.insert(vec![nums[i], nums[j], nums[l], nums[r]]);
                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {l += 1;}
                        r -= 1;
                        while l < r && nums[r] == nums[r + 1] {r -= 1;}
                    }
                    else if overflow || sum > target{
                        r -= 1;
                    }
                    else{
                        l += 1;
                    }
                }
            }
        }

        result.into_iter().collect::<Vec<Vec<i32>>>()
    }
}