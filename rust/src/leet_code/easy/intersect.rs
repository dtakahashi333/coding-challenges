// rust/src/leet_code/easy/intersect.rs

// 350. Intersection of Two Arrays II
// https://leetcode.com/problems/intersection-of-two-arrays-ii/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // Conditions:
        // * The given array is already sorted
        // * nums1's size is small compared to nums2's size
        // * elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let mut result = Solution::intersect(nums1, nums2);
        result.sort();
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let mut result = Solution::intersect(nums1, nums2);
        result.sort();
        assert_eq!(result, vec![4, 9]);
    }
}
