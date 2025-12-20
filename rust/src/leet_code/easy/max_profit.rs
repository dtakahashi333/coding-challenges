// rust/src/leet_code/easy/max_profit.rs

// 121. Best Time to Buy and Sell Stock
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(&self, prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 0..prices.len() - 1 {
            let buy_price = prices[i];
            for j in i + 1..prices.len() {
                let sell_price = prices[j];
                if profit < sell_price - buy_price {
                    profit = sell_price - buy_price;
                }
            }
        }
        profit
    }

    #[allow(dead_code)]
    pub fn max_profit2(&self, prices: Vec<i32>) -> i32 {
        let mut profit_max = 0;
        let mut price_min = prices[0];
        for i in 1..prices.len() {
            let price = prices[i];
            let profit = price - price_min;
            if profit > profit_max {
                profit_max = profit;
            }
            if price < price_min {
                price_min = price;
            }
        }
        profit_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let s = Solution;
        let result = s.max_profit(prices);
        assert_eq!(result, 5);
    }

    #[test]
    fn test2() {
        let prices = vec![7, 6, 4, 3, 1];
        let s = Solution;
        let result = s.max_profit(prices);
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let s = Solution;
        let result = s.max_profit2(prices);
        assert_eq!(result, 5);
    }

    #[test]
    fn test4() {
        let prices = vec![7, 6, 4, 3, 1];
        let s = Solution;
        let result = s.max_profit2(prices);
        assert_eq!(result, 0);
    }
}
