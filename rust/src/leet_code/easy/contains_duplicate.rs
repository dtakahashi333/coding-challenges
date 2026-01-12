// rust/src/leet_code/easy/contains_duplicate.rs

// 217. Contains Duplicate
// https://leetcode.com/problems/contains-duplicate/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut seen = HashSet::new();
        for n in nums {
            if seen.contains(&n) {
                return true;
            }
            seen.insert(n);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 1];
        let result = Solution::contains_duplicate(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::contains_duplicate(nums);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let result = Solution::contains_duplicate(nums);
        assert_eq!(result, true);
    }
}
