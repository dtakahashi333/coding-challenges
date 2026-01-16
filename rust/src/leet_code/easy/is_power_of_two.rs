// rust/src/leet_code/easy/is_power_of_two.rs

// 231. Power of Two
// https://leetcode.com/problems/power-of-two/description/

pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        match n {
            n if n < 0 => false,
            0 => false,
            1 => true,
            _ => {
                let mut x = n;
                while x > 1 {
                    if x % 2 != 0 {
                        return false;
                    }
                    x /= 2;
                }
                true
            }
        }
    }

    // The canonical way (fast & idiomatic)
    pub fn is_power_of_two2(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 1;
        let result = Solution::is_power_of_two(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let n = 16;
        let result = Solution::is_power_of_two(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let n = 3;
        let result = Solution::is_power_of_two(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let n = 0;
        let result = Solution::is_power_of_two(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let n = -16;
        let result = Solution::is_power_of_two(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test6() {
        let n = 1;
        let result = Solution::is_power_of_two2(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test7() {
        let n = 16;
        let result = Solution::is_power_of_two2(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test8() {
        let n = 3;
        let result = Solution::is_power_of_two2(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test9() {
        let n = 0;
        let result = Solution::is_power_of_two2(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test10() {
        let n = -16;
        let result = Solution::is_power_of_two2(n);
        assert_eq!(result, false);
    }
}
