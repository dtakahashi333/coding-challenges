// rust/src/leet_code/easy/contains_nearby_duplicate.rs

// 219. Contains Duplicate II
// https://leetcode.com/problems/contains-duplicate-ii/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let size = nums.len();
        let mut start = 0_usize;
        let mut end = start + k as usize;
        for i in start..end {
            for j in i + 1..=end {
                if i < size && j < size {
                    if nums[i] == nums[j] {
                        return true;
                    }
                }
            }
        }
        start += 1;
        end += 1;
        while end < size {
            for i in start..end {
                if nums[i] == nums[end] {
                    return true;
                }
            }
            start += 1;
            end += 1;
        }
        false
    }

    #[allow(dead_code)]
    pub fn contains_nearby_duplicate2(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashSet;

        let k = k as usize;
        let mut seen = HashSet::new();

        for (i, n) in nums.iter().enumerate() {
            if seen.contains(&n) {
                return true;
            }
            seen.insert(n);
            if seen.len() > k {
                seen.remove(&nums[i - k]);
            }
        }
        false
    }

    pub fn contains_nearby_duplicate3(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;

        let mut last_seen = HashMap::with_capacity(nums.len());
        let k = k as usize;

        for (i, &n) in nums.iter().enumerate() {
            if let Some(prev_index) = last_seen.get(&n) {
                if i - prev_index <= k {
                    return true;
                }
            }
            last_seen.insert(n, i);
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
        let k = 3;
        let result = Solution::contains_nearby_duplicate(nums, k);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        let result = Solution::contains_nearby_duplicate(nums, k);
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        let result = Solution::contains_nearby_duplicate(nums, k);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        let result = Solution::contains_nearby_duplicate2(nums, k);
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        let result = Solution::contains_nearby_duplicate2(nums, k);
        assert_eq!(result, true);
    }

    #[test]
    fn test6() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        let result = Solution::contains_nearby_duplicate2(nums, k);
        assert_eq!(result, false);
    }

    #[test]
    fn test7() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        let result = Solution::contains_nearby_duplicate3(nums, k);
        assert_eq!(result, true);
    }

    #[test]
    fn test8() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        let result = Solution::contains_nearby_duplicate3(nums, k);
        assert_eq!(result, true);
    }

    #[test]
    fn test9() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        let result = Solution::contains_nearby_duplicate3(nums, k);
        assert_eq!(result, false);
    }
}
