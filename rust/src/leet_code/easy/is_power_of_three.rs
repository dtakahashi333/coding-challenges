// rust/src/leet_code/easy/is_power_of_three.rs

// 326. Power of Three
// https://leetcode.com/problems/power-of-three/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut n = n;
        while n >= 3 {
            if n % 3 == 0 {
                n /= 3;
            } else {
                return false;
            }
        }
        n == 1
    }

    // By ChatGPT
    pub fn is_power_of_three2(n: i32) -> bool {
        // If n is a power of 3, it must divide the largest power of 3 evenly.
        n > 0 && 1162261467 % n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 27;
        let result = Solution::is_power_of_three(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let n = 0;
        let result = Solution::is_power_of_three(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let n = -1;
        let result = Solution::is_power_of_three(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let n = 9;
        let result = Solution::is_power_of_three(n);
        assert_eq!(result, true);
    }
    #[test]
    fn test5() {
        let n = 27;
        let result = Solution::is_power_of_three2(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test6() {
        let n = 0;
        let result = Solution::is_power_of_three2(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test7() {
        let n = -1;
        let result = Solution::is_power_of_three2(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test8() {
        let n = 9;
        let result = Solution::is_power_of_three2(n);
        assert_eq!(result, true);
    }
}
