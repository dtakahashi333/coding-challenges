// rust/src/leet_code/easy/can_construct.rs

// 383. Ransom Note
// https://leetcode.com/problems/ransom-note/description/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut char_stat: HashMap<char, i32> = HashMap::new();
        for letter in magazine.chars() {
            *char_stat.entry(letter).or_insert(0) += 1;
        }
        for letter in ransom_note.chars() {
            match char_stat.get_mut(&letter) {
                Some(count) => {
                    if *count <= 0 {
                        return false;
                    } else {
                        *count -= 1;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        true
    }

    pub fn can_construct2(ransom_note: String, magazine: String) -> bool {
        // use an array instead of a HashMap.
        let mut counts = [0; 26];

        for b in magazine.bytes() {
            counts[(b - b'a') as usize] += 1;
        }

        for b in ransom_note.bytes() {
            let index = (b - b'a') as usize;
            counts[index] -= 1;
            if counts[index] < 0 {
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
        let ransom_note = String::from("a");
        let magazine = String::from("b");
        let result = Solution::can_construct(ransom_note, magazine);
        assert!(!result);
    }

    #[test]
    fn test2() {
        let ransom_note = String::from("aa");
        let magazine = String::from("ab");
        let result = Solution::can_construct(ransom_note, magazine);
        assert!(!result);
    }

    #[test]
    fn test3() {
        let ransom_note = String::from("aa");
        let magazine = String::from("aab");
        let result = Solution::can_construct(ransom_note, magazine);
        assert!(result);
    }
    #[test]
    fn test4() {
        let ransom_note = String::from("a");
        let magazine = String::from("b");
        let result = Solution::can_construct2(ransom_note, magazine);
        assert!(!result);
    }

    #[test]
    fn test5() {
        let ransom_note = String::from("aa");
        let magazine = String::from("ab");
        let result = Solution::can_construct2(ransom_note, magazine);
        assert!(!result);
    }

    #[test]
    fn test6() {
        let ransom_note = String::from("aa");
        let magazine = String::from("aab");
        let result = Solution::can_construct2(ransom_note, magazine);
        assert!(result);
    }
}
