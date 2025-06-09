use std::cmp::min;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        //basically merge sort but without the merging and you only go halfway.
        //this should all look pretty familiar
        let len_is_even = (nums1.len() + nums2.len()) % 2 == 0;
        let half_len = (nums1.len() + nums2.len()) / 2 
            - if len_is_even {1} else {0};
        let (mut i, mut j) = (0, 0);
        let mut cur: f64 = 0.0;
        while i + j <= half_len{
            let (n1, n2) = (nums1.get(i), nums2.get(j));
            if n1.is_some() && n2.is_some(){
                let (n1, n2) = (n1.unwrap(), n2.unwrap());
                cur = (*min(n1, n2)).into();
                (i, j) = if n1 < n2 {(i + 1, j)} else {(i, j + 1)};
            }
            else if n1.is_some(){
                //because we're not merging, we can just stop while we're ahead and jump to the middle
                i = half_len - j;
                cur = nums1[i].into();
                i += 1;
                break;
            }
            else{
                j = half_len - i;
                cur = nums2[j].into();
                j += 1;
                break;
            }
        }
        
        //if the length is even, we gotta average the two middle values
        if len_is_even{
            let (n1, n2) = (nums1.get(i), nums2.get(j));
            if n1.is_some() && n2.is_some(){
                cur = (cur + *min(n1.unwrap(), n2.unwrap()) as f64) / 2.0;
            }
            else if n1.is_some(){
                cur = (cur + *n1.unwrap() as f64) / 2.0;
            }
            else{
                cur = (cur + *n2.unwrap() as f64) / 2.0;
            }
        }
        cur
    }
}