// rust/src/leet_code/easy/longest_common_prefix.rs

// 14. Longest Common Prefix
// https://leetcode.com/problems/longest-common-prefix/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(&self, strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let first_string = &strs[0];
        let mut prefix_len = 0;

        'outer: for (byte_index, c1) in first_string.char_indices() {
            for s in strs.iter().skip(1) {
                if byte_index >= s.len() {
                    break 'outer;
                }
                let c2 = s[byte_index..].chars().next().unwrap();
                if c2 != c1 {
                    break 'outer;
                }
            }
            prefix_len = byte_index + c1.len_utf8();
        }

        (&strs[0])[..prefix_len].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strs = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        let s = Solution;
        let result = s.longest_common_prefix(strs);
        assert_eq!(result, String::from("fl"));
    }

    #[test]
    fn test2() {
        let strs = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];
        let s = Solution;
        let result = s.longest_common_prefix(strs);
        assert_eq!(result, String::from(""));
    }
}
