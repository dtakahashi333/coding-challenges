// rust/src/leet_code/easy/intersection.rs

// 349. Intersection of Two Arrays
// https://leetcode.com/problems/intersection-of-two-arrays/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        for n in nums1 {
            set.insert(n);
        }
        let mut res = HashSet::new();
        for n in nums2 {
            if set.contains(&n) {
                res.insert(n);
            }
        }
        res.into_iter().collect()
    }

    pub fn intersection2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;

        let set1: HashSet<_> = nums1.into_iter().collect();
        let set2: HashSet<_> = nums2.into_iter().collect();

        set1.intersection(&set2).cloned().collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let mut result = Solution::intersection(nums1, nums2);
        result.sort();
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let mut result = Solution::intersection(nums1, nums2);
        result.sort();
        assert_eq!(result, vec![4, 9]);
    }

    #[test]
    fn test3() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let mut result = Solution::intersection2(nums1, nums2);
        result.sort();
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test4() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let mut result = Solution::intersection2(nums1, nums2);
        result.sort();
        assert_eq!(result, vec![4, 9]);
    }
}
