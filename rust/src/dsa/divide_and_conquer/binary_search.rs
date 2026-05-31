// rust/src/dsa/divide_and_conquer/binary_search.rs

pub struct Solution;

impl Solution {
    pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let size = nums.len();
        Self::binary_search_helper(nums, target, 0, size - 1)
    }

    fn binary_search_helper(nums: Vec<i32>, target: i32, start: usize, end: usize) -> i32 {
        if start == end {
            if nums[start] == target {
                return start as i32;
            } else {
                return -1;
            }
        }

        let middle = start + (end - start) / 2;

        if nums[middle] == target {
            middle as i32
        } else if nums[middle] > target {
            Self::binary_search_helper(nums, target, start, middle)
        } else {
            Self::binary_search_helper(nums, target, middle + 1, end)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 3, 5, 7, 9, 11];
        let target = 5;
        let result = Solution::binary_search(nums, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 3, 5, 7, 9, 11];
        let target = 6;
        let result = Solution::binary_search(nums, target);
        assert_eq!(result, -1);
    }

    #[test]
    fn test3() {
        let nums = vec![10];
        let target = 10;
        let result = Solution::binary_search(nums, target);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let nums = vec![10];
        let target = 5;
        let result = Solution::binary_search(nums, target);
        assert_eq!(result, -1);
    }

    #[test]
    fn test5() {
        let nums = vec![];
        let target = 3;
        let result = Solution::binary_search(nums, target);
        assert_eq!(result, -1);
    }

    #[test]
    fn test6() {
        let nums = vec![2, 4, 6, 8, 10];
        let target = 2;
        let result = Solution::binary_search(nums, target);
        assert_eq!(result, 0);
    }

    #[test]
    fn test7() {
        let nums = vec![2, 4, 6, 8, 10];
        let target = 10;
        let result = Solution::binary_search(nums, target);
        assert_eq!(result, 4);
    }
}
