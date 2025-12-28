// rust/src/leet_code/easy/majority_element.rs

// 169. Majority Element
// https://leetcode.com/problems/majority-element/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn majority_element(&self, nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            map.entry(n).or_insert(0);
            if let Some(v) = map.get_mut(&n) {
                *v += 1;
            }
        }

        *map.iter().max_by_key(|(_, v)| *v).map(|(k, _)| k).unwrap() as i32
    }

    #[allow(dead_code)]
    pub fn majority_element2(&self, nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        nums[nums.len() / 2]
    }

    // Boyer–Moore Voting Algorithm
    pub fn majority_element3(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;

        for num in nums {
            if count == 0 {
                candidate = num;
            }
            count += if num == candidate { 1 } else { -1 };
        }

        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 2, 3];
        let s = Solution;
        let result = s.majority_element(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let s = Solution;
        let result = s.majority_element(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let nums = vec![3, 2, 3];
        let s = Solution;
        let result = s.majority_element2(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test4() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let s = Solution;
        let result = s.majority_element2(nums);
        assert_eq!(result, 2);
    }
}
