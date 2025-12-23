// rust/src/leet_code/easy/is_palindrome.rs

// 125. Valid Palindrome
// https://leetcode.com/problems/valid-palindrome/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(&self, s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        // covert String to Vec<char>
        let s_vec: Vec<char> = s.to_lowercase().chars().collect();
        let mut left = 0_usize;
        let mut right = s_vec.len() - 1;
        while left < right {
            if !s_vec[left].is_alphanumeric() {
                left += 1;
            } else if !s_vec[right].is_alphanumeric() {
                right -= 1;
            } else {
                if s_vec[left] != s_vec[right] {
                    return false;
                }
                left += 1;
                right -= 1;
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
        let phrase = String::from("A man, a plan, a canal: Panama");
        let s = Solution;
        let result = s.is_palindrome(phrase);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let phrase = String::from("race a car");
        let s = Solution;
        let result = s.is_palindrome(phrase);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let phrase = String::from(" ");
        let s = Solution;
        let result = s.is_palindrome(phrase);
        assert_eq!(result, true);
    }
}
