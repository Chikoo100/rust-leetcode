use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, 1);
        let mut profit = 0;

        while r < prices.len() {
            let curr_prof = prices[r] - prices[l];

            profit = cmp::max(profit, curr_prof);

            if curr_prof < 0 {
                l = r;
                r += 1;
            }
            else {
                r += 1;
            }
        }

        return profit;

        }
}
