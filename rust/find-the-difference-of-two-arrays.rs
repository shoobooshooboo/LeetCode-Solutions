use std::collections::HashSet;
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        //convert to hashsets to remove redundant elements
        let nums1: HashSet<i32> = nums1.into_iter().collect();
        let nums2: HashSet<i32> = nums2.into_iter().collect();
        let mut result = vec![Vec::new(), Vec::new()];
        
        //get the differences
        for n in nums1.iter(){
            if !nums2.contains(n){
                result[0].push(*n);
            }
        }
        for n in nums2.iter(){
            if !nums1.contains(n){
                result[1].push(*n);
            }
        }

        result
    }
}