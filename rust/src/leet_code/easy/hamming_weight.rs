// rust/src/leet_code/easy/hamming_weight.rs

// 191. Number of 1 Bits
// https://leetcode.com/problems/number-of-1-bits/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn hamming_weight(n: i32) -> i32 {
        let mut n = n;
        let mut weight = 0;
        while n > 0 {
            weight += n & 1;
            n >>= 1;
        }
        weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 11;
        let result = Solution::hamming_weight(n);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let n = 128;
        let result = Solution::hamming_weight(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let n = 2147483645;
        let result = Solution::hamming_weight(n);
        assert_eq!(result, 30);
    }
}
