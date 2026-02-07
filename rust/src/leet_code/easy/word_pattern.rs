// rust/src/leet_code/easy/word_pattern.rs

// 290. Word Pattern
// https://leetcode.com/problems/word-pattern/description/

use std::collections::HashMap;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split_whitespace().collect();
        let chars: Vec<char> = pattern.chars().collect();
        if words.len() != chars.len() {
            return false;
        }
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        for (ch, &word) in chars.iter().zip(words.iter()) {
            map1.entry(ch).or_insert(word);
            if let Some(&w) = map1.get(&ch) {
                if w != word {
                    return false;
                }
            }
            map2.entry(word).or_insert(ch);
            if let Some(&c) = map2.get(word) {
                if c != ch {
                    return false;
                }
            }
        }
        true
    }

    // More idiomatic Rust version (recommended) by Rust
    pub fn word_pattern2(pattern: String, s: String) -> bool {
        let mut map1: HashMap<char, &str> = HashMap::new();
        let mut map2: HashMap<&str, char> = HashMap::new();

        let mut words = s.split_whitespace();
        let mut chars = pattern.chars();

        loop {
            match (chars.next(), words.next()) {
                (Some(ch), Some(word)) => {
                    if map1.get(&ch).is_some_and(|&w| w != word) {
                        return false;
                    }
                    if map2.get(word).is_some_and(|&c| c != ch) {
                        return false;
                    }

                    map1.insert(ch, word);
                    map2.insert(word, ch);
                }
                (None, None) => break,
                _ => return false, // length mismatch
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
        let pattern = String::from("abba");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat fish");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let pattern = String::from("aaaa");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let pattern = String::from("abca");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern2(pattern, s);
        assert_eq!(result, true);
    }

    #[test]
    fn test6() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat fish");
        let result = Solution::word_pattern2(pattern, s);
        assert_eq!(result, false);
    }

    #[test]
    fn test7() {
        let pattern = String::from("aaaa");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern2(pattern, s);
        assert_eq!(result, false);
    }

    #[test]
    fn test8() {
        let pattern = String::from("abca");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern2(pattern, s);
        assert_eq!(result, false);
    }
}
