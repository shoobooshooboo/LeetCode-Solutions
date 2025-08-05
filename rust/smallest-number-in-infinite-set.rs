struct SmallestInfiniteSet {
    cur: i32,
    added: BinaryHeap<i32>, //array sorted in decreasing order
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use std::collections::BinaryHeap;
impl SmallestInfiniteSet {

    fn new() -> Self {
        Self{
            cur: 0,
            added: BinaryHeap::new()
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        -self.added.pop().unwrap_or_else(|| {self.cur += 1; -(self.cur)})
    }
    
    fn add_back(&mut self, num: i32) {
        if num <= self.cur && self.added.iter().all(|&n| -n != num){
            self.added.push(-num);
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */