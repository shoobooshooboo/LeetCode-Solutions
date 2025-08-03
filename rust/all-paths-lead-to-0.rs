use std::collections::HashSet;
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        //naive, obvious solution. O(n^2) time (gross).
        let mut can_reach_0 = HashSet::with_capacity(n as usize);
        can_reach_0.insert(0);
        let mut count = 0;
        let connections: Vec<(i32, i32)> = connections.into_iter().map(|pair| (pair[0], pair[1])).collect();
        while can_reach_0.len() < n as usize{
            for (a, b) in connections.iter(){
                if can_reach_0.contains(b){
                    can_reach_0.insert(*a);
                }else if can_reach_0.contains(a){
                    count += 1;
                    can_reach_0.insert(*b);
                }
            }
        }

        count
    }
}