// rust/src/leet_code/easy/missing_number.rs

// 268. Missing Number
// https://leetcode.com/problems/missing-number/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut flags = vec![false; n + 1];
        for num in nums {
            flags[num as usize] = true;
        }

        flags.iter().position(|&x| x == false).unwrap() as i32
    }

    // Avoid extra memory (O(1) space) by ChatGPT
    pub fn missing_number2(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let expected = n * (n + 1) / 2;
        let actual: i32 = nums.iter().sum();
        expected - actual
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 0, 1];
        let result = Solution::missing_number(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 1];
        let result = Solution::missing_number(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let result = Solution::missing_number(nums);
        assert_eq!(result, 8);
    }

    #[test]
    fn test4() {
        let nums = vec![3, 0, 1];
        let result = Solution::missing_number2(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test5() {
        let nums = vec![0, 1];
        let result = Solution::missing_number2(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test6() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let result = Solution::missing_number2(nums);
        assert_eq!(result, 8);
    }
}
