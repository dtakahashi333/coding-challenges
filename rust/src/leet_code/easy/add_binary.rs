// rust/src/leet_code/easy/add_binary.rs

// 67. Add Binary
// https://leetcode.com/problems/add-binary/description/

use std::cmp;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn add_binary(&self, a: String, b: String) -> String {
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();

        let a_len = a.len();
        let b_len = b.len();
        let max_len = cmp::max(a_len, b_len);

        let mut carry = 0;
        let mut result = Vec::new();
        // Rust-style / Idiomatic Improvements
        // for (digit_a, digit_b) in a.chars().rev().zip_longest(b.chars().rev()) { ... }
        for i in 0..max_len {
            let digit_a = if i < a_len {
                a[a_len - 1 - i].to_digit(2).unwrap()
            } else {
                0
            };
            let digit_b = if i < b_len {
                b[b_len - 1 - i].to_digit(2).unwrap()
            } else {
                0
            };
            let sum = digit_a + digit_b + carry;
            result.push(std::char::from_digit(sum % 2, 2).unwrap());
            carry = sum / 2;
        }

        if carry > 0 {
            result.push(std::char::from_digit(carry, 2).unwrap());
        }

        result.reverse();
        result.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = "11".to_string();
        let b = "1".to_string();
        let s = Solution;
        let result = s.add_binary(a, b);
        assert_eq!(result, "100");
    }

    #[test]
    fn test2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let s = Solution;
        let result = s.add_binary(a, b);
        assert_eq!(result, "10101");
    }
}
