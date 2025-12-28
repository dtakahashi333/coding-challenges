// rust/src/leet_code/easy/convert_to_title.rs

// 168. Excel Sheet Column Title
// https://leetcode.com/problems/excel-sheet-column-title/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn convert_to_title(&self, column_number: i32) -> String {
        use std::collections::VecDeque;
        let mut char_list: VecDeque<char> = VecDeque::new();
        let mut num = column_number;
        while num > 0 {
            num -= 1;
            let code = num % 26 + 65;
            char_list.push_front(code as u8 as char);
            num /= 26;
        }
        char_list.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let column_number = 1;
        let s = Solution;
        let result = s.convert_to_title(column_number);
        assert_eq!(result, "A");
    }

    #[test]
    fn test2() {
        let column_number = 28;
        let s = Solution;
        let result = s.convert_to_title(column_number);
        assert_eq!(result, "AB");
    }

    #[test]
    fn test3() {
        let column_number = 701;
        let s = Solution;
        let result = s.convert_to_title(column_number);
        assert_eq!(result, "ZY");
    }
}
