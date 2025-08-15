impl Solution {
    pub fn min_cost_climbing_stairs(mut costs: Vec<i32>) -> i32 {
        //using costs as the dp array cause why make a second array 
        //when we're given ownership of costs?

        //basically walk down each step (except the top two) and add the
        //cost of the current step to the lower of the two next steps.
        for i in (0..costs.len() - 2).rev(){
            costs[i] += costs[i + 1].min(costs[i + 2]);
        }

        //then the bottom two steps will have their min cost,
        //so just choose the min of them
        costs[0].min(costs[1])
    }
}