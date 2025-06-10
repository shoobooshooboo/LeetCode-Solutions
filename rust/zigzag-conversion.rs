use std::collections::HashMap;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        //elegent solution with iterators
        let mut result: Vec<(i32, char)>= (0..num_rows) // 0,1,2,3
            .chain((1..num_rows - 1).rev()) //0,1,2,3,2,1
            .cycle() //0,1,2,3,2,1,0,1...
            .zip(s.chars()) //effectively enumerating the input 
            .collect();

        //sort by row
        result.sort_by_key(|&(row, _)| row);
        
        //remove rows
        result.into_iter().map(|(_, c)| c).collect()
    }
}