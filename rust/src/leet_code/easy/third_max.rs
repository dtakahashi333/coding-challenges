// rust/src/leet_code/easy/third_max.rs

// 414. Third Maximum Number
// https://leetcode.com/problems/third-maximum-number/description/

pub struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut first = i64::MIN;
        for n in &nums {
            if first < *n as i64 {
                first = *n as i64;
            }
        }
        let mut second = i64::MIN;
        for n in &nums {
            if first > *n as i64 && second < *n as i64 {
                second = *n as i64;
            }
        }
        let mut third = i64::MIN;
        for n in &nums {
            if second > *n as i64 && third < *n as i64 {
                third = *n as i64;
            }
        }
        if third == i64::MIN {
            first as i32
        } else {
            third as i32
        }
    }

    pub fn third_max2(nums: Vec<i32>) -> i32 {
        let mut first: Option<i32> = None;
        let mut second: Option<i32> = None;
        let mut third: Option<i32> = None;

        for n in nums {
            // skip duplicates
            if first == Some(n) || second == Some(n) || third == Some(n) {
                continue;
            }

            if first.is_none() || n > first.unwrap() {
                third = second;
                second = first;
                first = Some(n);
            } else if second.is_none() || n > second.unwrap() {
                third = second;
                second = Some(n);
            } else if third.is_none() || n > third.unwrap() {
                third = Some(n);
            }
        }

        third.unwrap_or(first.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = [3, 2, 1];
        let result = Solution::third_max(nums.into());
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let nums = [1, 2];
        let result = Solution::third_max(nums.into());
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let nums = [2, 2, 3, 1];
        let result = Solution::third_max(nums.into());
        assert_eq!(result, 1);
    }

    #[test]
    fn test4() {
        let nums = [1, 1, 2];
        let result = Solution::third_max(nums.into());
        assert_eq!(result, 2);
    }

    #[test]
    fn test5() {
        let nums = [3, 2, 1];
        let result = Solution::third_max2(nums.into());
        assert_eq!(result, 1);
    }

    #[test]
    fn test6() {
        let nums = [1, 2];
        let result = Solution::third_max2(nums.into());
        assert_eq!(result, 2);
    }

    #[test]
    fn test7() {
        let nums = [2, 2, 3, 1];
        let result = Solution::third_max2(nums.into());
        assert_eq!(result, 1);
    }

    #[test]
    fn test8() {
        let nums = [1, 1, 2];
        let result = Solution::third_max2(nums.into());
        assert_eq!(result, 2);
    }
}
