use std::collections::HashMap;
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        //literally just two-sum but repeatedly in the same loop.
        let mut counts = HashMap::<i32, u32>::new();
        let mut operations = 0;
        for n in nums{
            if n >= k{
                continue;
            }
            let complement = k - n;

            if let Some(c) = counts.get_mut(&n){
                *c += 1;
            }
            else{
                counts.insert(n, 1);
            }

            if let Some(c) = counts.get_mut(&complement){
                if n == complement{
                    if *c > 1{
                        *c -= 2;
                        operations += 1;
                    }
                }else{
                    if *c > 0{
                        *c -= 1;
                        *counts.get_mut(&n).unwrap() -= 1;
                        operations += 1;
                    }
                }
            }
        }

        operations
    }
}