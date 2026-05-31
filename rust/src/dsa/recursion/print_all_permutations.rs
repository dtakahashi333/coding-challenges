// rust/src/dsa/recursion/permutation.rs

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn print_all_permutations(nums: &[i32]) {
        let mut map = vec![false; nums.len()];
        let mut deque = VecDeque::new();
        Self::helper(nums, &mut deque, &mut map);
    }

    fn helper(nums: &[i32], deque: &mut VecDeque<i32>, map: &mut Vec<bool>) {
        if !map.contains(&false) {
            println!("{:?}", deque);
            return;
        }

        for i in 0..nums.len() {
            if !map[i] {
                deque.push_back(nums[i]);
                map[i] = true;
                Self::helper(nums, deque, map);
                deque.pop_back();
                map[i] = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = [1, 2, 3];
        Solution::print_all_permutations(&nums);
    }

    #[test]
    fn test2() {
        let nums = [];
        Solution::print_all_permutations(&nums);
    }

    #[test]
    fn test3() {
        let nums = [1];
        Solution::print_all_permutations(&nums);
    }
}
