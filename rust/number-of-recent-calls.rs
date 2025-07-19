use std::collections::VecDeque;
struct RecentCounter {
    pings: VecDeque<i32> 
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        Self{
            pings: VecDeque::new(),
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.pings.push_back(t);

        while self.pings.front().is_some_and(|&p| p < t - 3000){
            self.pings.pop_front();
        }

        self.pings.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */