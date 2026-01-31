// rust/src/leet_code/easy/is_anagram.rs

// 242. Valid Anagram
// https://leetcode.com/problems/valid-anagram/description/

use std::collections::HashMap;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::new();
        for (i, &c) in s.iter().enumerate() {
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
            if let Some(&c2) = t.get(i) {
                map.entry(c2).and_modify(|v| *v -= 1).or_insert(-1);
            } else {
                return false;
            }
        }

        map.retain(|_, v| *v != 0);
        map.len() == 0
    }

    pub fn is_anagram2(s: &str, t: &str) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counts: HashMap<char, i32> = HashMap::new();

        for (a, b) in s.chars().zip(t.chars()) {
            *counts.entry(a).or_insert(0) += 1;
            *counts.entry(b).or_insert(0) -= 1;
        }

        counts.values().all(|&v| v == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        let result = Solution::is_anagram(s, t);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let s = String::from("rat");
        let t = String::from("car");
        let result = Solution::is_anagram(s, t);
        assert_eq!(result, false);
    }
}
