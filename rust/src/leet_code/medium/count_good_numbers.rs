// rust/src/leet_code/medium/count_good_numbers.rs

// 1922. Count Good Numbers
// https://leetcode.com/problems/count-good-numbers/description/

pub struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn count_good_numbers(n: i64) -> i32 {
        let even_count = (n + 1) / 2;
        let odd_count = n / 2;
        (Self::mod_pow(5, even_count) * Self::mod_pow(4, odd_count) % Self::MOD) as i32
    }

    fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
        let mut result = 1;

        while exp > 0 {
            if exp % 2 == 1 {
                result = (base * result) % Self::MOD;
            }

            base = (base * base) % Self::MOD;

            exp /= 2;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 1;
        let result = Solution::count_good_numbers(n);
        assert_eq!(result, 5);
    }

    #[test]
    fn test2() {
        let n = 4;
        let result = Solution::count_good_numbers(n);
        assert_eq!(result, 400);
    }

    #[test]
    fn test3() {
        let n = 50;
        let result = Solution::count_good_numbers(n);
        assert_eq!(result, 564908303);
    }

    #[test]
    fn test4() {
        let n = 806166225460393;
        let result = Solution::count_good_numbers(n);
        assert_eq!(result, 643535977);
    }
}
