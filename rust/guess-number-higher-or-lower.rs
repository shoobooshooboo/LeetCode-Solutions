/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut low, mut high) = (1, n);
        let mut mid;
        loop{
            mid = ((high as i64 + low as i64) / 2) as i32;
            match guess(mid){
                -1 => high = mid - 1,
                1 => low = mid + 1,
                _ => break,
            }
        }

        mid
    }
}