impl Solution {
    pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
        //sliding window solution (kinda. more like an inchworm).
        let (mut i, mut j) = (0, 0);
        let mut max = 0;
        loop{
            //extend the head of the inchworm to cover the max number of 0s
            while k > 0{
                if let Some(n) = nums.get(j){
                    if *n == 0{
                        k -= 1;
                    }
                    j += 1;
                }
                else{
                    break;
                }
            }

            //now get the remaining 1s
            while let Some(n) = nums.get(j){
                if *n != 1{
                    break;
                }
                j += 1;
            }

            //j is one ahead of the last one, so j - i will produce the proper length
            max = max.max(j - i);
            if j >= nums.len(){
                break;
            }

            //now move the inchworm but to get past the first 0
            while nums.get(i).is_some_and(|n| *n == 1){
                i += 1;
            }
            i += 1;
            k += 1;
        }
        max as i32
    }
}