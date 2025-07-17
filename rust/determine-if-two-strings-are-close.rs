use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        //frequency maps
        let mut freq_map1 = HashMap::new();
        word1.chars().map(|c| {*freq_map1.entry(c).or_insert_with(|| 0) += 1;}).last();
        let mut freq_map2 = HashMap::new();
        word2.chars().map(|c| {*freq_map2.entry(c).or_insert_with(|| 0) += 1;}).last();

        //now ensure that the same characters are present and the same frequencies are present
        let mut values1: Vec<u32> = freq_map1.values().map(|n| *n).collect();
        let mut values2: Vec<u32> = freq_map2.values().map(|n| *n).collect();
        values1.sort();
        values2.sort();
        let keys1: HashSet<char> = freq_map1.keys().map(|c| *c).collect();
        let keys2: HashSet<char> = freq_map2.keys().map(|c| *c).collect();

        values1 == values2 && keys1 == keys2
    }
}