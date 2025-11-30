// rust/src/leet_code/easy/str_str.rs

// 28. Find the Index of the First Occurrence in a String
// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn str_str(&self, haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        let haystack_chars: Vec<char> = haystack.chars().collect();
        let needle_chars: Vec<char> = needle.chars().collect();
        let haystack_len = haystack.chars().count();
        let needle_len = needle.chars().count();

        for (i, c1) in haystack_chars.iter().enumerate() {
            if *c1 == needle_chars[0] {
                let mut ii = i;
                let mut count = 0;
                for c2 in needle_chars.iter() {
                    if ii >= haystack_len || *c2 != haystack_chars[ii] {
                        break;
                    }
                    count += 1;
                    ii += 1;
                }
                if count == needle_len {
                    return i as i32;
                }
            }
        }
        // for (i, _) in haystack_chars.iter().enumerate() {
        //     // Make sure there's enough chars left to match needle
        //     if i + needle_len > haystack_chars.len() {
        //         break;
        //     }

        //     if haystack_chars[i..i + needle_len] == needle_chars[..] {
        //         return i as i32; // i is now character index
        //     }
        // }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        let s = Solution;
        let result = s.str_str(haystack, needle);
        assert_eq!(result, 0);
    }

    #[test]
    fn test2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();
        let s = Solution;
        let result = s.str_str(haystack, needle);
        assert_eq!(result, -1);
    }

    #[test]
    fn test3() {
        let haystack = "aaa".to_string();
        let needle = "aaaa".to_string();
        let s = Solution;
        let result = s.str_str(haystack, needle);
        assert_eq!(result, -1);
    }
}
