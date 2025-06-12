use std::cmp::max;
impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        //two pointers method. fairly intuitive
        let (mut i, mut j) = (0, heights.len() - 1);
        let mut max_area = 0;

        while i < j{
            //get area and move the shorter pointer
            let area = (j - i) as i32 * 
                if heights[i] < heights[j] {i += 1; heights[i - 1]} 
                else {j -= 1; heights[j + 1]};
            //update max area
            max_area = max(max_area, area);
        }
        max_area
    }
}