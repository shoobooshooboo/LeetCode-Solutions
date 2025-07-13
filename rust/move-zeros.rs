impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        //two pointer approach
        let (mut i, mut j) = (0, 1);
        loop{
            //i will go to the leftmost 0
            while nums.get(i).is_some_and(|x| *x != 0){
                i += 1;
            }
            //j will go to the leftmost non-zero to the right of the leftmost 0
            if j <= i{
                j = i + 1;
            }
            while nums.get(j).is_some_and(|x| *x == 0){
                j += 1;
            }

            //if we cant swap, then we stop
            if j >= nums.len(){
                break;
            }

            //swap
            nums.swap(i, j);
        }
    }
}