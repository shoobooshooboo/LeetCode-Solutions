use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Side{
    Right,
    Left,
}

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let candidates = candidates as usize;
        if candidates * 2 >= costs.len(){
            let len = costs.len();
            let heap: BinaryHeap<i64> = costs.into_iter().map(|x| x as i64).collect();
            return heap.into_sorted_vec().into_iter().take(k as usize).sum::<i64>();
        }
        let mut heap = BinaryHeap::with_capacity(candidates * 2);
        let (mut i, mut j) = (0, costs.len() - 1);
        while i < candidates{
            heap.push((-costs[i], Side::Left));
            i += 1;
            heap.push((-costs[j], Side::Right));
            j -= 1;
        }

        let mut total = 0i64;
        for _ in 0..k{
            let (val, side) = heap.pop().unwrap();
            total += -val as i64;
            if i <= j{
                if side == Side::Left{
                    heap.push((-costs[i], Side::Left));
                    i += 1;
                }else{
                    heap.push((-costs[j], Side::Right));
                    j -= 1;
                }
            }
        }
        total
    }
}