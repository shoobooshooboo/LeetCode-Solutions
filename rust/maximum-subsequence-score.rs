use std::collections::BinaryHeap;
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        //makes more sense as a usize
        let k = k as usize;
        //sort both arrays by nums2
        let mut num_pairs: Vec<(i64, i64)> = nums2.into_iter().zip(nums1.into_iter()).map(|(a, b)| (a as i64, b as i64)).collect();
        num_pairs.sort();
        let num_pairs = num_pairs; //DO NOT MODIFY IT ANYMORE
        
        let mut heap = BinaryHeap::<i64>::new();
        let mut prefix_sum: i64 = 0;
        let mut max: i64 = 0;
        for (a, b) in num_pairs.into_iter().rev(){
            prefix_sum += b;
            heap.push(-b);

            if heap.len() > k{
                prefix_sum += heap.pop().unwrap();
            }
            if heap.len() == k{
                max = max.max(prefix_sum * a);
            }
        }

        max
    }

}