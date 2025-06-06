use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //using a hashmap makes this process linear
        let mut complements = HashMap::new();

        //enumerate because we want to return indexes
        for (i, n) in nums.iter().enumerate(){
            //check if the complement of the current num has been seen
            match complements.get(n){
                Some(&c) => return vec![i as i32, c as i32], //we're done
                None => () //we're not done
            };

            //map current complement to index of current number
            complements.insert(target - n, i);
        }

        //we're assuming there's exactly one solution, 
        //so this should never be reached
        vec![]
    }
}