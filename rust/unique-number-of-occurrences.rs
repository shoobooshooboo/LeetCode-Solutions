use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        //frequency map
        let mut freq = HashMap::new();
        
        //populate frequency map
        for n in arr{
            if let Some(occ) = freq.get_mut(&n){
                *occ += 1;
            }else{
                freq.insert(n, 1);
            }
        }

        //get a set of the occurrences
        let occurrences: HashSet<&i32> = freq.iter().map(|(k, v)| v).collect();
        //if theres the name number of unique elements as unique frequencies of elements, return true
        occurrences.len() == freq.len()
    }
}