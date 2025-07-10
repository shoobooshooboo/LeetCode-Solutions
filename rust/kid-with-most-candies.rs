impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().max().unwrap().clone();
        candies.into_iter().map(|x| x + extra_candies >= max).collect()
    }
}