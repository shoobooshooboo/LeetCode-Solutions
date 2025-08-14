impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        //binary search can be used on an unsorted array to find the min or the max of the array.
        //in this case, we just need the max of the array.
        let (mut low, mut high) = (0, nums.len() - 1);
        while low < high{
            let mid = (low + high) / 2;
            if nums[mid] < nums[mid + 1]{
                low = mid + 1;
            }else{
                high = mid;
            }
        }

        low as i32
    }
}