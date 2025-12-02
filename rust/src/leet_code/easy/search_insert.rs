// rust/src/leet_code/easy/search_insert.rs

// 35. Search Insert Position
// https://leetcode.com/problems/search-insert-position/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search_insert(&self, nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut start = 0;
        let mut end = nums.len();
        while start < end {
            let mid = start + (end - start) / 2;
            if nums[mid] < target {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        start as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 5, 6];
        let target = 5;
        let s = Solution;
        let result = s.search_insert(nums, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let s = Solution;
        let result = s.search_insert(nums, target);
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let s = Solution;
        let result = s.search_insert(nums, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn test4() {
        let nums = vec![1, 3];
        let target = 0;
        let s = Solution;
        let result = s.search_insert(nums, target);
        assert_eq!(result, 0);
    }
}
