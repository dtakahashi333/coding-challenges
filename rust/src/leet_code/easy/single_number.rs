// rust/src/leet_code/easy/single_number.rs

// 136. Single Number
// https://leetcode.com/problems/single-number/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn single_number(&self, nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for num in &nums {
            if let Some(count) = map.get_mut(num) {
                if *count == 0 {
                    *count = 1;
                } else {
                    *count = 0;
                }
            } else {
                map.insert(*num, 1);
            }
        }
        map.into_iter()
            .find(|(_, v)| *v == 1)
            .map(|(k, _)| k)
            .unwrap()
        // for (k, v) in &map {
        //     if *v != 0 {
        //         return *k;
        //     }
        // }
        // -3 * 10_i32.pow(4)
        // unreachable!();
    }

    pub fn single_number2(&self, nums: Vec<i32>) -> i32 {
        let mut accumulated_value = 0;
        for num in nums {
            accumulated_value ^= num;
        }
        accumulated_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 2, 1];
        let s = Solution;
        let result = s.single_number(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let nums = vec![4, 1, 2, 1, 2];
        let s = Solution;
        let result = s.single_number(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test3() {
        let nums = vec![1];
        let s = Solution;
        let result = s.single_number(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test4() {
        let nums = vec![2, 2, 1];
        let s = Solution;
        let result = s.single_number2(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test5() {
        let nums = vec![4, 1, 2, 1, 2];
        let s = Solution;
        let result = s.single_number2(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test6() {
        let nums = vec![1];
        let s = Solution;
        let result = s.single_number2(nums);
        assert_eq!(result, 1);
    }
}
