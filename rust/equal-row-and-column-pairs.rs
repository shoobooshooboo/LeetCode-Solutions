use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        //frequency map
        let mut row_freq = HashMap::new();
        grid.iter().for_each(|row| {*row_freq.entry(row.clone()).or_insert(0) += 1;});
        
        //now just check every column against the rows. add the frequency of that row to the total count.
        let n = grid.len();
        let mut pairs = 0;
        for i in 0..n{
            let mut col = Vec::with_capacity(n);
            for j in 0..n{
                col.push(grid[j][i]);
            }
            if let Some(&count) = row_freq.get(&col){
                pairs += count;
            }
        }

        pairs
    }
}