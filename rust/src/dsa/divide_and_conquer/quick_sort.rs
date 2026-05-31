// rust/src/dsa/divide_and_conquer/quick_sort.rs

pub struct Solution;

impl Solution {
    pub fn quick_sort(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }

        let pivot = nums[nums.len() - 1];
        let mut left = Vec::new();
        let mut pivots = Vec::new();
        let mut right = Vec::new();

        for num in &nums {
            if *num == pivot {
                pivots.push(*num);
            } else if *num < pivot {
                left.push(*num);
            } else {
                right.push(*num);
            }
        }

        left = Self::quick_sort(left);
        right = Self::quick_sort(right);

        left.append(&mut pivots);
        left.append(&mut right);

        left
    }

    pub fn quick_sort2(nums: &mut [i32]) {
        if nums.len() <= 1 {
            return;
        }
        let pivot_index = Self::partition(nums);

        let (left, right) = nums.split_at_mut(pivot_index);

        Self::quick_sort2(left);
        Self::quick_sort2(&mut right[1..]);
    }

    fn partition(nums: &mut [i32]) -> usize {
        let pivot = nums[nums.len() - 1];

        let mut i = 0_usize;

        for j in 0..nums.len() {
            if nums[j] <= pivot {
                nums.swap(i, j);
                i += 1;
            }
        }

        i - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![4, 2, 1, 3];
        let result = Solution::quick_sort(nums);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test2() {
        let nums = vec![-1, 5, 3, 4, 0];
        let result = Solution::quick_sort(nums);
        assert_eq!(result, vec![-1, 0, 3, 4, 5]);
    }

    #[test]
    fn test3() {
        let nums = vec![];
        let result = Solution::quick_sort(nums);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test4() {
        let nums = vec![1];
        let result = Solution::quick_sort(nums);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test5() {
        let nums = vec![2, 1];
        let result = Solution::quick_sort(nums);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test6() {
        let nums = vec![1, 1, 1];
        let result = Solution::quick_sort(nums);
        assert_eq!(result, vec![1, 1, 1]);
    }

    #[test]
    fn test7() {
        let mut nums = [4, 2, 1, 3];
        Solution::quick_sort2(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4]);
    }

    #[test]
    fn test8() {
        let mut nums = [-1, 5, 3, 4, 0];
        Solution::quick_sort2(&mut nums);
        assert_eq!(nums, [-1, 0, 3, 4, 5]);
    }

    #[test]
    fn test9() {
        let mut nums = [];
        Solution::quick_sort2(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test10() {
        let mut nums = [1];
        Solution::quick_sort2(&mut nums);
        assert_eq!(nums, [1]);
    }

    #[test]
    fn test11() {
        let mut nums = [2, 1];
        Solution::quick_sort2(&mut nums);
        assert_eq!(nums, [1, 2]);
    }

    #[test]
    fn test12() {
        let mut nums = [1, 1, 1];
        Solution::quick_sort2(&mut nums);
        assert_eq!(nums, [1, 1, 1]);
    }

    #[test]
    fn test13() {
        let mut nums = [10, 5];
        Solution::quick_sort2(&mut nums);
        assert_eq!(nums, [5, 10]);
    }
    #[test]
    fn test14() {
        let mut nums = [3, 2, 1];
        Solution::quick_sort2(&mut nums);
        assert_eq!(nums, [1, 2, 3]);
    }
}
