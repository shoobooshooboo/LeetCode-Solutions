use std::collections::HashSet;
use std::collections::HashMap;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //solution very similar to 2sum using a map of complements
        let mut complements: HashMap<i32,usize> = nums.iter().enumerate().map(|(i, n)| (-n, i)).collect();
        let mut result = HashSet::new();

        for i in 0..nums.len(){
            for j in (i + 1)..nums.len(){
                if let Some((n, k)) = complements.get_key_value(&(nums[i] + nums[j])){
                    if *k != i && *k != j{
                        let mut triplet = vec![-n, nums[i], nums[j]];
                        triplet.sort();
                        result.insert(triplet);
                    }
                }
            }
        }
        result.into_iter().collect()
    }
}