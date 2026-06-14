// rust/src/leet_code/easy/my_pow.rs

// 50. Pow(x, n)
// https://leetcode.com/problems/powx-n/description/

pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let is_neg_exp = n < 0;
        let n = n as i64;
        let result = Self::my_pow_helper(x, n.abs());
        if is_neg_exp { 1_f64 / result } else { result }
    }

    fn my_pow_helper(x: f64, n: i64) -> f64 {
        match n {
            0 => 1_f64,
            _ => x * Self::my_pow_helper(x, n - 1),
        }
    }

    pub fn my_pow2(x: f64, n: i32) -> f64 {
        let is_neg_exp = n < 0;
        let mut n = n as i64;
        n = n.abs();
        let mut result = 1_f64;
        loop {
            match n {
                0 => break,
                _ => {
                    result *= x;
                    n -= 1;
                }
            }
        }
        if is_neg_exp { 1_f64 / result } else { result }
    }

    pub fn my_pow3(x: f64, n: i32) -> f64 {
        let is_neg_exp = n < 0;
        let n = n as i64;
        let result = Self::my_pow3_helper(x, n.abs());
        if is_neg_exp { 1_f64 / result } else { result }
    }

    fn my_pow3_helper(x: f64, n: i64) -> f64 {
        match n {
            _ if n <= 0 => 1_f64,
            _ => {
                let mut n = n;
                let is_odd_exp = n % 2 != 0;
                if is_odd_exp {
                    n -= 1;
                }
                let mut result = Self::my_pow3_helper(x, n / 2);
                result *= result;
                if is_odd_exp { result * x } else { result }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let x = 2.00000;
        let n = 10;
        let eps = 1e-10;
        assert!((Solution::my_pow(x, n) - 1024.00000).abs() < eps);
        assert!((Solution::my_pow2(x, n) - 1024.00000).abs() < eps);
        assert!((Solution::my_pow3(x, n) - 1024.00000).abs() < eps);
    }

    #[test]
    fn test2() {
        let x = 2.10000;
        let n = 3;
        let eps = 1e-10;
        assert!((Solution::my_pow(x, n) - 9.26100).abs() < eps);
        assert!((Solution::my_pow2(x, n) - 9.26100).abs() < eps);
        assert!((Solution::my_pow3(x, n) - 9.26100).abs() < eps);
    }

    #[test]
    fn test3() {
        let x = 2.00000;
        let n = -2;
        let eps = 1e-10;
        assert!((Solution::my_pow(x, n) - 0.25000).abs() < eps);
        assert!((Solution::my_pow2(x, n) - 0.25000).abs() < eps);
        assert!((Solution::my_pow3(x, n) - 0.25000).abs() < eps);
    }

    #[test]
    fn test4() {
        let x = 2.00000;
        let n = -200000000;
        let eps = 1e-10;
        // assert!((Solution::my_pow(x, n) - 0.00000).abs() < eps);
        assert!((Solution::my_pow2(x, n) - 0.00000).abs() < eps);
        assert!((Solution::my_pow3(x, n) - 0.00000).abs() < eps);
    }

    #[test]
    fn test5() {
        let x = 2.00000;
        let n = -2147483648;
        let eps = 1e-10;
        // assert!((Solution::my_pow(x, n) - 0.00000).abs() < eps);
        assert!((Solution::my_pow2(x, n) - 0.00000).abs() < eps);
        assert!((Solution::my_pow3(x, n) - 0.00000).abs() < eps);
    }
}
