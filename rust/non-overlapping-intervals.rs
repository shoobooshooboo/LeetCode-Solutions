impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        //sort the array so that intervals are ordered by starting point and then length
        intervals.sort();
        
        let mut count = 0;
        let mut max = i32::MIN; //maximum value in the interval
        for inter in intervals{
            //if the max of the running interval completely envelop this new interval, get rid of the big interval
            if max > inter[1]{
                count += 1;
                max = inter[1];
            }
            //if the max only partially envelops the new interval, get rid of the new one
            else if max > inter[0]{
                count += 1;
            }
            //otherwise, just set the new max
            else{
                max = inter[1];
            }
        }
        count
    }
}