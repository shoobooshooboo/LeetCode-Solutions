impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        //sort the ballons by interval
        points.sort_unstable();

        //it will take at least one shot (points is guaranteed to be non-zero in size)
        let mut shots = 1;
        //now get the minimum range that you could shoot in
        let (mut min, mut max) = (i32::MIN, i32::MAX);
        for interval in points{
            //restrict the range 
            min = min.max(interval[0]);
            max = max.min(interval[1]);
            //if the range isn't valid anymore, then that's all the ones that'll be contained in that shot.
            if min > max{
                shots += 1;
                min = interval[0];
                max = interval[1];
            }
        }
        shots
    }
}