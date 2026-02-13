// rust/src/leet_code/easy/is_power_of_four.rs

// 342. Power of Four
// https://leetcode.com/problems/power-of-four/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let mut n = n;
        while n > 1 {
            if n % 4 == 0 {
                n /= 4;
            } else {
                return false;
            }
        }
        n > 0 && n == 1
    }

    pub fn is_power_of_four2(n: i32) -> bool {
        // Check if the number is a power of four.
        if n > 0 && n & (n - 1) == 0 {
            // Check the 1-bit position
            // Mask for 32-bit integers
            let mask = 0b0101_0101_0101_0101_0101_0101_0101_0101;
            return n & mask != 0;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 16;
        let result = Solution::is_power_of_four(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let n = 5;
        let result = Solution::is_power_of_four(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let n = 1;
        let result = Solution::is_power_of_four(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let n = 16;
        let result = Solution::is_power_of_four2(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let n = 5;
        let result = Solution::is_power_of_four2(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test6() {
        let n = 1;
        let result = Solution::is_power_of_four2(n);
        assert_eq!(result, true);
    }
}
