impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        //dynamic programming solution with two dp arrays.
        let mut max_sell = vec![0; prices.len()];
        let mut max_buy = vec![0; prices.len()];
        max_sell[0] = 0;
        max_buy[0] = -prices[0] - fee;
        for i in 1..prices.len(){
            //buy a new one if the previous sell price justifies it
            max_buy[i] = max_buy[i - 1].max(max_sell[i - 1] - prices[i] - fee);
            //sell the current one if the previous buy justifies it
            max_sell[i] = max_sell[i - 1].max(max_buy[i - 1] + prices[i]);
        }

        *max_sell.last().unwrap()
    }
}