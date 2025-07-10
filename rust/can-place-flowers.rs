impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
        //handle edge case.
        if n == 0 {
            return true;
        }

        //change the flowerbed to a map of bools cause its easier to work with. 0 == true in this.
        let available: Vec<bool> = flowerbed.into_iter().map(|x| x == 0).collect();

        //handle two other edge cases.
        if available.len() <= 2{
            return available[0] && *available.get(1).unwrap_or(&true) && n < 2;
        }

        //we're doing a sliding-window type thing.
        let (mut cur, mut prev) = (available[1], available[0]);
        // if we can set the first one, do that now.
        if prev && cur {
            prev = true;
            n -= 1;
        }

        //now go through the whole list
        for next in available.into_iter().skip(2){
            // if we've placed all the flowers, awesome. we're done.
            if n == 0 { return true; }

            // if all three spots are available, decrement the remaining flower count (n) and set cur to false to show it's taken.
            if prev && cur && next{
                n -= 1;
                cur = false;
            }

            //update prev and cur.
            prev = cur;
            cur = next;
        }

        //if the last one can be placed, do that.
        if prev && cur{
            n -= 1;
        }

        //return whether there are more flowers to place.
        n == 0
    }
}