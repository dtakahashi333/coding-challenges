// rust/src/leet_code/easy/merge.rs

// 88. Merge Sorted Array
// https://leetcode.com/problems/merge-sorted-array/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge(&self, nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;
        let nums1_copy = nums1.clone();
        let mut i = 0_usize;
        let mut j = 0_usize;
        let mut k = 0_usize;
        while i < m || j < n {
            if i < m && j < n {
                if nums1_copy[i] < nums2[j] {
                    nums1[k] = nums1_copy[i];
                    i += 1;
                } else {
                    nums1[k] = nums2[j];
                    j += 1;
                }
            } else if i < m {
                nums1[k] = nums1_copy[i];
                i += 1;
            } else if j < n {
                nums1[k] = nums2[j];
                j += 1;
            }
            k += 1;
        }
    }

    #[allow(dead_code)]
    pub fn merge2(&self, nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = (m - 1) as isize;
        let mut j = (n - 1) as isize;
        let mut k = (m + n - 1) as isize;
        while k >= 0 {
            if i >= 0 && j >= 0 {
                if nums1[i as usize] < nums2[j as usize] {
                    nums1[k as usize] = nums2[j as usize];
                    j -= 1;
                } else {
                    nums1[k as usize] = nums1[i as usize];
                    i -= 1;
                }
            } else if i >= 0 {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else if j >= 0 {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        let s = Solution;
        s.merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        let s = Solution;
        s.merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        let s = Solution;
        s.merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test4() {
        let mut nums1 = vec![2, 0];
        let m = 1;
        let mut nums2 = vec![1];
        let n = 1;
        let s = Solution;
        s.merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2]);
    }
    #[test]
    fn test5() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        let s = Solution;
        s.merge2(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test6() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        let s = Solution;
        s.merge2(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test7() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        let s = Solution;
        s.merge2(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test8() {
        let mut nums1 = vec![2, 0];
        let m = 1;
        let mut nums2 = vec![1];
        let n = 1;
        let s = Solution;
        s.merge2(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2]);
    }
}
