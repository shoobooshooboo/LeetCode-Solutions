use std::cmp::Ordering;
impl Solution {
    pub fn successful_pairs(mut spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        //sort potions and then binary search for the lowest index of the potion with sufficient power.
        potions.sort_unstable();
        let success = success as f64;
        let mut result = Vec::with_capacity(spells.len());

        for spell in spells{
            let target = (success / spell as f64).ceil() as i32;
            result.push((
                potions.len() 
                - Self::binary_search_closest(&potions, target).unwrap_or(potions.len())
                ) as i32);
        }

        result
    }

    fn binary_search_closest(arr: &Vec<i32>, target: i32) -> Option<usize>{
        //binary search for the lowest index number greater than or equal to the target number.
        let (mut low, mut high) = (0, arr.len() - 1);
        if arr[high] < target{
            return None;
        }
        let mut result = 100000;

        while low <= high{
            if arr[low] >= target{
                return Some(low);
            }
            let mid = (low + high) / 2;

            match arr[mid].cmp(&target){
                Ordering::Less => low = mid + 1,
                Ordering::Equal | Ordering::Greater => {result = mid; high = mid - 1;},
            }
        }

        Some(result)
    }
}