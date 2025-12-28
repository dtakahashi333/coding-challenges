// rust/src/leet_code/easy/reverse_bits.rs

// 190. Reverse Bits
// https://leetcode.com/problems/reverse-bits/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_bits(n: i32) -> i32 {
        let mut n = n;
        let mut bits = Vec::new();
        for _ in 0..32 {
            bits.push(n % 2);
            n /= 2;
        }
        bits.iter().fold(0, |acc, &d| acc * 2 + d)
    }

    #[allow(dead_code)]
    pub fn reverse_bits2(n: i32) -> i32 {
        let mut n = n;
        let mut bits = Vec::new();
        for _ in 0..32 {
            bits.push(n & 1);
            n >>= 1;
        }
        bits.iter().fold(0, |acc, &d| acc * 2 + d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 43261596;
        let result = Solution::reverse_bits(n);
        assert_eq!(result, 964176192);
    }

    #[test]
    fn test2() {
        let n = 2147483644;
        let result = Solution::reverse_bits(n);
        assert_eq!(result, 1073741822);
    }

    #[test]
    fn test3() {
        let n = 43261596;
        let result = Solution::reverse_bits2(n);
        assert_eq!(result, 964176192);
    }

    #[test]
    fn test4() {
        let n = 2147483644;
        let result = Solution::reverse_bits2(n);
        assert_eq!(result, 1073741822);
    }
}
