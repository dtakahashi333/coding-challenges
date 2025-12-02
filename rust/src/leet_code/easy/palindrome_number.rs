// rust/src/leet_code/easy/palindrome_number.rs

// 9. Palindrome Number
// https://leetcode.com/problems/palindrome-number/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(&self, x: i32) -> bool {
        let x_as_string = x.to_string();
        let chars: Vec<char> = x_as_string.chars().collect();
        let string_length = chars.len();
        for i in 0..string_length / 2 {
            if chars[i] != chars[string_length - 1 - i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let x = 121;
        let s = Solution;
        let result = s.is_palindrome(x);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let x = -121;
        let s = Solution;
        let result = s.is_palindrome(x);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let x = 10;
        let s = Solution;
        let result = s.is_palindrome(x);
        assert_eq!(result, false);
    }
}
