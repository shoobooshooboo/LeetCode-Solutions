impl Solution {
    pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
        //do binary search with possible rates
        let (mut low, mut high) = (1, *piles.iter().max().unwrap());

        //this part is just binary search
        let mut result = 0;
        while low <= high{
            let rate = (high + low) / 2;

            //instead of comparing mid to some target that you're looking for, just check if it works as a rate
            let mut fast_enough = true;
            let mut hours = h;
            let mut i = 0;
            for &p in piles.iter(){
                if hours <= 0{
                    fast_enough = false;
                    break;
                }
                hours -= 1 + (p - 1) / rate;
            }
            if hours < 0{
                fast_enough = false;
            }

            if fast_enough{
                result = rate;
                high = rate - 1;
            }else{
                low = rate + 1;
            }
        }

        result
    }
}