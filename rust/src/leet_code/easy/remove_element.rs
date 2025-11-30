// rust/src/leet_code/easy/remove_element.rs

// 27. Remove Element
// https://leetcode.com/problems/remove-element/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn remove_element(&self, nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0 as usize;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
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
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let s = Solution;
        let k = s.remove_element(&mut nums, val);
        assert_eq!(k, 2);
        let expected = vec![2, 2];
        for i in 0..k as usize {
            assert_eq!(nums[i], expected[i]);
        }
    }

    #[test]
    fn test2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let s = Solution;
        let k = s.remove_element(&mut nums, val);
        nums[0..k as usize].sort();
        assert_eq!(k, 5);
        let mut expected = vec![0, 1, 4, 0, 3];
        expected.sort();
        for i in 0..k as usize {
            println!("{}, {}", nums[i], expected[i]);
        }
        for i in 0..k as usize {
            assert_eq!(nums[i], expected[i]);
        }
    }
}
