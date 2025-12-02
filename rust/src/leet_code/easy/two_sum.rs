// rust/src/leet_code/easy/two_sum.rs

// 1. Two Sum
// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result = Vec::new();
        for (i, num) in nums.iter().enumerate() {
            let diff = target - *num;
            if map.contains_key(&diff) {
                result.push(i as i32);
                let index = *map.get(&diff).unwrap() as i32;
                result.push(index);
            } else {
                map.insert(num, i);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let s = Solution;
        let mut result = s.two_sum(nums, target);
        result.sort();
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let s = Solution;
        let mut result = s.two_sum(nums, target);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test3() {
        let nums = vec![3, 3];
        let target = 6;
        let s = Solution;
        let mut result = s.two_sum(nums, target);
        result.sort();
        assert_eq!(result, vec![0, 1]);
    }
}
