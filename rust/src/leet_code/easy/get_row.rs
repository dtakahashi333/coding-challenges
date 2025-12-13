// rust/src/leet_code/easy/get_row.rs

// 119. Pascal's Triangle II
// https://leetcode.com/problems/pascals-triangle-ii/description/

use num_bigint::BigUint;
use num_traits::One;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn get_row(&self, row_index: i32) -> Vec<i32> {
        let row_index = row_index as u64;
        let mut row = Vec::new();
        for i in 0..=row_index {
            row.push(match i32::try_from(Solution::comb(row_index, i)) {
                Ok(v) => v,
                Err(_) => panic!("BigInt does not fit into i32"),
            });
        }
        row
    }

    #[allow(dead_code)]
    fn perm(n: u64, k: u64) -> BigUint {
        let mut prod = BigUint::one();
        let mut n = n;
        for _ in 0..k {
            prod *= n;
            n -= 1;
        }
        prod
    }

    #[allow(dead_code)]
    fn comb(n: u64, k: u64) -> BigUint {
        // Solution::fact(n) / Solution::fact(k) / Solution::fact(n - k)
        Solution::perm(n, k) / Solution::perm(k, k)
    }

    #[allow(dead_code)]
    fn fact(n: u64) -> u64 {
        if n == 0 {
            return 1;
        }
        let mut prod = 1;
        for i in 1..=n {
            prod *= i;
        }
        prod
    }

    #[allow(dead_code)]
    pub fn get_row2(&self, row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;

        let mut row = vec![1; row_index + 1];
        let mut val = 1_usize;

        for i in 1..row_index {
            val = val * (row_index - (i - 1)) / i;
            row[i] = val as i32;
        }
        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let row_index = 3;
        let s = Solution;
        let result = s.get_row(row_index);
        assert_eq!(result, vec![1, 3, 3, 1]);
    }

    #[test]
    fn test2() {
        let row_index = 0;
        let s = Solution;
        let result = s.get_row(row_index);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test3() {
        let row_index = 1;
        let s = Solution;
        let result = s.get_row(row_index);
        assert_eq!(result, vec![1, 1]);
    }

    #[test]
    fn test4() {
        let row_index = 13;
        let s = Solution;
        let result = s.get_row(row_index);
        assert_eq!(
            result,
            vec![
                1, 13, 78, 286, 715, 1287, 1716, 1716, 1287, 715, 286, 78, 13, 1
            ]
        );
    }

    #[test]
    fn test5() {
        let row_index = 21;
        let s = Solution;
        let result = s.get_row(row_index);
        assert_eq!(
            result,
            vec![
                1, 21, 210, 1330, 5985, 20349, 54264, 116280, 203490, 293930, 352716, 352716,
                293930, 203490, 116280, 54264, 20349, 5985, 1330, 210, 21, 1
            ]
        );
    }
    #[test]
    fn test6() {
        let row_index = 3;
        let s = Solution;
        let result = s.get_row2(row_index);
        assert_eq!(result, vec![1, 3, 3, 1]);
    }

    #[test]
    fn test7() {
        let row_index = 0;
        let s = Solution;
        let result = s.get_row2(row_index);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test8() {
        let row_index = 1;
        let s = Solution;
        let result = s.get_row2(row_index);
        assert_eq!(result, vec![1, 1]);
    }

    #[test]
    fn test9() {
        let row_index = 13;
        let s = Solution;
        let result = s.get_row2(row_index);
        assert_eq!(
            result,
            vec![
                1, 13, 78, 286, 715, 1287, 1716, 1716, 1287, 715, 286, 78, 13, 1
            ]
        );
    }

    #[test]
    fn test10() {
        let row_index = 21;
        let s = Solution;
        let result = s.get_row2(row_index);
        assert_eq!(
            result,
            vec![
                1, 21, 210, 1330, 5985, 20349, 54264, 116280, 203490, 293930, 352716, 352716,
                293930, 203490, 116280, 54264, 20349, 5985, 1330, 210, 21, 1
            ]
        );
    }
}
