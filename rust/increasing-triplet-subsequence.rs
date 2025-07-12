impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        //the first two in the sequence.
        let (mut i, mut j) = (i32::MAX, i32::MAX);

        for n in nums{
            //i will be the lowest, so if theres a new min found, i will get it first 
            if n <= i {
                i = n;
            //if it's greater than i but lower than j, than j can get it.
            }else if n <= j{
                j = n;
            //if it's greater than both of them, then theres a triplet.
            }else{
                return true;
            }
        }

        false
    }
}