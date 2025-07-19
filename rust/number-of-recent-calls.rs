struct RecentCounter {
    pings: Vec<i32> 
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        Self{
            pings: Vec::new(),
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.pings.push(t.clone());
        let mut count = 0;
        for &p in self.pings.iter().rev(){
            if p <= t && p >= t - 3000{
                count += 1;
            } else { break; }
        }

        count
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */