impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        //the number of uniques also doubles as the index to place new ones in
        let mut unique_count = 0;
        for i in 0..nums.len(){
            //using .get() to safely index out of bounds
            if nums.get(i) == nums.get(i + 1){
                continue;
            }

            //set that index and then update the unique count
            nums[unique_count] = nums[i];
            unique_count += 1;
        }

        unique_count as i32
    }
}