// rust/src/leet_code/eash/length_of_last_word.rs

// 58. Length of Last Word
// https://leetcode.com/problems/length-of-last-word/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn length_of_last_word(&self, s: String) -> i32 {
        // let words: Vec<char> = s.chars().collect();
        // let words_len = words.len();
        // let mut count = 0;
        // for i in 0..words_len {
        //     if words[words_len - 1 - i] != ' ' {
        //         count += 1;
        //     } else if count > 0 {
        //         break;
        //     }
        // }
        let mut count = 0;
        for c in s.chars().rev() {
            if c != ' ' {
                count += 1;
            } else if count > 0 {
                break;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let words_str = "Hello World".to_string();
        let s = Solution;
        let result = s.length_of_last_word(words_str);
        assert_eq!(result, 5);
    }

    #[test]
    fn test2() {
        let words_str = "   fly me   to   the moon  ".to_string();
        let s = Solution;
        let result = s.length_of_last_word(words_str);
        assert_eq!(result, 4);
    }

    #[test]
    fn test3() {
        let words_str = "luffy is still joyboy".to_string();
        let s = Solution;
        let result = s.length_of_last_word(words_str);
        assert_eq!(result, 6);
    }
}
