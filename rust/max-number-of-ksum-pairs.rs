impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        //two pointer solution.
        //first, sort the array
        nums.sort();
        //get your two pointers
        let (mut i, mut j) = (0, nums.len() - 1);
        let mut operations = 0;
        while i < j{
            //check the sum. update i, j, and operations as necessary
            let sum = nums[i] + nums[j];
            if sum == k{
                i += 1;
                j -= 1;
                operations += 1;
            } else if sum > k{
                j -= 1;
            } else{
                i += 1;
            }
        }

        operations
    }
}