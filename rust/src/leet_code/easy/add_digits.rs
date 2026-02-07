// rust/src/leet_code/easy/add_digits.rs

// 258. Add Digits
// https://leetcode.com/problems/add-digits/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;

        while num >= 10 {
            let mut sum = 0;
            loop {
                sum += num % 10;
                num /= 10;
                if num == 0 {
                    break;
                }
            }
            num = sum;
        }

        num
    }

    // The "Digital Root" Approach (O(1)) by Gemini
    pub fn add_digits2(num: i32) -> i32 {
        if num == 0 { 0 } else { 1 + (num - 1) % 9 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let num = 38;
        let result = Solution::add_digits(num);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let num = 0;
        let result = Solution::add_digits(num);
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let num = 38;
        let result = Solution::add_digits2(num);
        assert_eq!(result, 2);
    }

    #[test]
    fn test4() {
        let num = 0;
        let result = Solution::add_digits2(num);
        assert_eq!(result, 0);
    }
}
