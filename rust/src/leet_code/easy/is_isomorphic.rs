// rust/src/leet_code/easy/is_isomorphic.rs

// 205. Isomorphic Strings
// https://leetcode.com/problems/isomorphic-strings/description/

use std::collections::HashMap;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        if s.len() != t.len() {
            return false;
        }

        let mut map1: HashMap<char, char> = HashMap::new();
        let mut map2: HashMap<char, char> = HashMap::new();

        for i in 0..s.len() {
            if map1.contains_key(&s[i]) && map1[&s[i]] != t[i] {
                return false;
            }
            if map2.contains_key(&t[i]) && map2[&t[i]] != s[i] {
                return false;
            }
            map1.insert(s[i], t[i]);
            map2.insert(t[i], s[i]);
        }
        true
    }

    pub fn is_isomorphic2(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();

        for (cs, ct) in s.chars().zip(t.chars()) {
            if let Some(&mapped) = map1.get(&cs) {
                if mapped != ct {
                    return false;
                }
            }

            if let Some(&mapped) = map2.get(&ct) {
                if mapped != cs {
                    return false;
                }
            }

            map1.insert(cs, ct);
            map2.insert(ct, cs);
        }

        true
    }

    // By ChatGPT
    pub fn is_isomorphic3(s: String, t: String) -> bool {
        let mut last_s = [0i32; 256];
        let mut last_t = [0i32; 256];

        for (i, (cs, ct)) in s.bytes().zip(t.bytes()).enumerate() {
            let i = i as i32 + 1;

            if last_s[cs as usize] != last_t[ct as usize] {
                return false;
            }

            last_s[cs as usize] = i;
            last_t[ct as usize] = i;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("egg");
        let t = String::from("add");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let s = String::from("foo");
        let t = String::from("bar");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let s = String::from("paper");
        let t = String::from("title");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, true);
    }
}
