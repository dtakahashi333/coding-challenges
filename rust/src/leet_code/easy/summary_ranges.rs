// rust/src/leet_code/easy/summary_ranges.rs

// 228. Summary Ranges
// https://leetcode.com/problems/summary-ranges/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ranges: Vec<String> = Vec::new();
        if nums.is_empty() {
            return ranges;
        }

        let mut start = nums[0] as i64;
        let mut prev = nums[0] as i64;
        for n in nums.iter().skip(1) {
            let x = *n as i64;
            if x as i64 - prev > 1 {
                if start == prev {
                    ranges.push(start.to_string());
                } else {
                    ranges.push(format!("{}->{}", start, prev));
                }
                start = x;
            }
            prev = x;
        }

        // don't forget the last range
        if start == prev {
            ranges.push(start.to_string());
        } else {
            ranges.push(format!("{}->{}", start, prev));
        }

        ranges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let result = Solution::summary_ranges(nums);
        assert_eq!(result, vec!["0->2", "4->5", "7"]);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        let result = Solution::summary_ranges(nums);
        assert_eq!(result, vec!["0", "2->4", "6", "8->9"]);
    }

    #[test]
    fn test3() {
        let nums = vec![-2147483648, 0, 2, 3, 4, 6, 8, 9];
        let result = Solution::summary_ranges(nums);
        assert_eq!(result, vec!["-2147483648", "0", "2->4", "6", "8->9"]);
    }
}
