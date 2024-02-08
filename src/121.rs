impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut profit, mut buy):(i32,i32)=(0,prices[0]);
        let mut i:i32=0;
        for i in 1..prices.len() {
            profit=profit.max(prices[i]-buy);
            buy = buy.min(prices[i]);
        }
        profit
    }
}