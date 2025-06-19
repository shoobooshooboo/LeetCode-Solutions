impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        //nums -> iter -> dereference elements -> filter elements -> collect
        *nums = nums.into_iter().map(|n| *n).filter(|n| *n != val).collect();

        nums.len() as i32
    }
}