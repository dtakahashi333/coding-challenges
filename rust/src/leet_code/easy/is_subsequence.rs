// rust/src/leet_code/easy/is_subsequence.rs

// 392. Is Subsequence
// https://leetcode.com/problems/is-subsequence/description/

pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let mut s_iter = s.bytes();
        let mut current_target = s_iter.next();

        for b in t.bytes() {
            if Some(b) == current_target {
                current_target = s_iter.next();
            }
            if current_target.is_none() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        let result = Solution::is_subsequence(s, t);
        assert!(result);
    }

    #[test]
    fn test2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        let result = Solution::is_subsequence(s, t);
        assert!(!result);
    }
}
