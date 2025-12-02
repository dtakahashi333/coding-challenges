// rust/src/leet_code/easy/my_sqrt.rs

// 69. Sqrt(x)
// https://leetcode.com/problems/sqrtx/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn my_sqrt(&self, x: i32) -> i32 {
        let x = x as i64;
        let mut i = 0 as i64;
        while i * i <= x {
            i += 1;
        }
        i as i32 - 1
    }

    #[allow(dead_code)]
    pub fn my_sqrt2(&self, x: i32) -> i32 {
        let x = x as i64;
        let mut left = 1 as i64;
        let mut right = x;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_sq = mid * mid;
            if mid_sq == x {
                return mid as i32;
            } else if mid_sq < x {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        right as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let x = 4;
        let s = Solution;
        let result = s.my_sqrt(x);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let x = 8;
        let s = Solution;
        let result = s.my_sqrt(x);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let x = 4;
        let s = Solution;
        let result = s.my_sqrt2(x);
        assert_eq!(result, 2);
    }

    #[test]
    fn test4() {
        let x = 8;
        let s = Solution;
        let result = s.my_sqrt2(x);
        assert_eq!(result, 2);
    }
}
