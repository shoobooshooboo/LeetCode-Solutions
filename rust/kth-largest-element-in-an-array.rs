use std::collections::BinaryHeap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        //literally just using a binary heap as intended.
        let mut heap: BinaryHeap<i32> = nums.into_iter().collect();
        for _ in 1..k{
            heap.pop();
        }
        heap.pop().unwrap()
    }
}