// rust/src/leet_code/easy/count_bits.rs

// 338. Counting Bits
// https://leetcode.com/problems/counting-bits/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for mut x in 0..=n {
            let mut bits = 0;
            while x > 0 {
                bits += x & 1;
                x >>= 1;
            }
            res.push(bits);
        }
        res
    }

    pub fn count_bits2(n: i32) -> Vec<i32> {
        let mut res = Vec::new();
        res.push(0);
        for x in 1..=n {
            res.push(res[(x >> 1) as usize] + (x & 1));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 2;
        let result = Solution::count_bits(n);
        assert_eq!(result, vec![0, 1, 1]);
    }

    #[test]
    fn test2() {
        let n = 5;
        let result = Solution::count_bits(n);
        assert_eq!(result, vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn test3() {
        let n = 2;
        let result = Solution::count_bits2(n);
        assert_eq!(result, vec![0, 1, 1]);
    }

    #[test]
    fn test4() {
        let n = 5;
        let result = Solution::count_bits2(n);
        assert_eq!(result, vec![0, 1, 1, 2, 1, 2]);
    }
}
