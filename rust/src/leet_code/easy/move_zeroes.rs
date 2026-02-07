// rust/src/leet_code/easy/move_zeroes.rs

// 283. Move Zeroes
// https://leetcode.com/problems/move-zeroes/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut cur = 0_usize;
        let mut tail = 0_usize;
        while cur < nums.len() {
            if let Some(&num) = nums.get(cur) {
                if num != 0 {
                    // swap values
                    nums[cur] = nums[tail];
                    nums[tail] = num;
                    tail += 1;
                }
            }
            cur += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}
