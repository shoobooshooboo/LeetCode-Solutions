struct StockSpanner {
    //daily prices is just a monotonic stack of strictly non-increasing numbers
    daily_prices: Vec<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        Self{
            daily_prices: Vec::new()
        }
    }
    
    fn next(&mut self, price: i32) -> i32 {
        //this is basic monotonic stack stuff
        let mut count = 1;
        while let Some((p, days)) = self.daily_prices.pop(){
            if price < p{
                self.daily_prices.push((p, days));
                break;
            }
            count += days;
        }
        self.daily_prices.push((price, count.clone()));
        count
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */