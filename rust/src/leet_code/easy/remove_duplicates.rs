// rust/src/leet_code/easy/remove_duplicates.rs

// 26. Remove Duplicates from Sorted Array
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(&self, nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut k = 1; // size
        let mut prev = nums[0];
        // let mut i = 1; // current
        // while i < nums.len() {
        //     while i < nums.len() && nums[i] == prev {
        //         i += 1;
        //     }
        //     if i < nums.len() {
        //         nums[k] = nums[i];
        //         prev = nums[k];
        //         k += 1;
        //     }
        //     i += 1;
        // }
        for i in 1..nums.len() {
            if nums[i] != prev {
                nums[k] = nums[i];
                prev = nums[i];
                k += 1;
            }
        }

        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![1, 1, 2];
        let s = Solution;
        let result = s.remove_duplicates(&mut nums);
        assert_eq!(result, 2);
        let expected = vec![1, 2];
        for i in 0..result as usize {
            assert_eq!(nums[i], expected[i]);
        }
    }

    #[test]
    fn test2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let s = Solution;
        let result = s.remove_duplicates(&mut nums);
        assert_eq!(result, 5);
        let expected = vec![0, 1, 2, 3, 4];
        for i in 0..result as usize {
            assert_eq!(nums[i], expected[i]);
        }
    }
}
