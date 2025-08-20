impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut count = 0;
        while a | b != c{
            if c & 1 == 1{
                if a & 1 == 0 && b & 1 == 0{
                    count += 1;
                }
            }else{
                count += (a & 1) + (b & 1);
            }

            a = a >> 1;
            b = b >> 1;
            c = c >> 1;
        }
        count
    }
}