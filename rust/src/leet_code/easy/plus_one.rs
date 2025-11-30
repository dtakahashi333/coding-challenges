// rust/src/leet_code/easy/plus_one.rs

// 66. Plus One
// https://leetcode.com/problems/plus-one/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn plus_one(&self, digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 1;
        for d in digits.iter_mut().rev() {
            let new_digit = *d + carry;
            *d = new_digit % 10;
            carry = new_digit / 10;
        }
        if carry > 0 {
            digits.insert(0, carry);
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let digits = vec![1, 2, 3];
        let s = Solution;
        let result = s.plus_one(digits);
        let expected = vec![1, 2, 4];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let digits = vec![4, 3, 2, 1];
        let s = Solution;
        let result = s.plus_one(digits);
        let expected = vec![4, 3, 2, 2];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let digits = vec![9];
        let s = Solution;
        let result = s.plus_one(digits);
        let expected = vec![1, 0];
        assert_eq!(result, expected);
    }
}
