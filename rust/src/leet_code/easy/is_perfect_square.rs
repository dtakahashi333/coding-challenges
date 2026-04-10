// rust/src/leet_code/easy/is_perfect_square.rs

// 367. Valid Perfect Square
// https://leetcode.com/problems/valid-perfect-square/description/

pub struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut num = num;
        let upper_bound = (num as f64).sqrt() as i32;
        for i in 2..=upper_bound {
            let mut count = 0;
            while num % i == 0 {
                num = num / i;
                count += 1;
            }
            if count % 2 != 0 {
                return false;
            }
        }

        if num > 1 {
            return false;
        }

        true
    }

    // 👇 Prime checking inside loop is wasteful
    #[allow(dead_code)]
    fn is_prime(num: i32) -> bool {
        // 1 and below are not prime
        if num <= 1 {
            return false;
        }

        // Check factors from 2 up to sqrt(num)
        for i in 2..=((num as f64).sqrt() as i32) {
            if num % i == 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let num = 16;
        let result = Solution::is_perfect_square(num);
        assert!(result);
    }

    #[test]
    fn test2() {
        let num = 14;
        let result = Solution::is_perfect_square(num);
        assert!(!result);
    }

    #[test]
    fn test3() {
        let num = 5;
        let result = Solution::is_perfect_square(num);
        assert!(!result);
    }
}
