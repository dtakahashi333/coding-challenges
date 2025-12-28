// rust/src/leet_code/easy/title_to_number.rs

// 171. Excel Sheet Column Number
// https://leetcode.com/problems/excel-sheet-column-number/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn title_to_number(column_title: String) -> i32 {
        let mut sum = 0;
        for (_, c) in column_title.char_indices() {
            let num = c as i32 - 64;
            sum = sum * 26 + num;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let column_title = String::from("A");
        let result = Solution::title_to_number(column_title);
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let column_title = String::from("AB");
        let result = Solution::title_to_number(column_title);
        assert_eq!(result, 28);
    }

    #[test]
    fn test3() {
        let column_title = String::from("ZY");
        let result = Solution::title_to_number(column_title);
        assert_eq!(result, 701);
    }
}
