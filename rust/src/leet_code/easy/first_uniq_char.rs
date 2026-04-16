// rust/src/leet_code/easy/first_uniq_char.rs

// 387. First Unique Character in a String
// https://leetcode.com/problems/first-unique-character-in-a-string/description/

pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counts = [0; 26];
        // 1. Count the number of the character appearances.
        for b in s.bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        // 2. Find the first character which appears only once in the string.
        for (i, b) in s.bytes().enumerate() {
            if counts[(b - b'a') as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("leetcode");
        let result = Solution::first_uniq_char(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn test2() {
        let s = String::from("loveleetcode");
        let result = Solution::first_uniq_char(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let s = String::from("aabb");
        let result = Solution::first_uniq_char(s);
        assert_eq!(result, -1);
    }
}
