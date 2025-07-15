impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        //just find the greatest prefix sum.
        let mut max = 0;
        gain.into_iter().fold(
            0, |mut height, x| {
                height += x;
                max = max.max(height);
                height
            });
        max
    }
}