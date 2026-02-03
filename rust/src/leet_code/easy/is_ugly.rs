// rust/src/leet_code/easy/is_ugly.rs

// 263. Ugly Number
// https://leetcode.com/problems/ugly-number/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let mut n = n;

        let prime_factors = [2, 3, 5];

        for factor in prime_factors {
            while n > 1 && n % factor == 0 {
                n /= factor;
            }
        }

        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 6;
        let result = Solution::is_ugly(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let n = 1;
        let result = Solution::is_ugly(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let n = 14;
        let result = Solution::is_ugly(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let n = -1;
        let result = Solution::is_ugly(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let n = 0;
        let result = Solution::is_ugly(n);
        assert_eq!(result, false);
    }
}