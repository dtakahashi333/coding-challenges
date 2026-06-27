// rust/src/leet_code/easy/lemonade_change.rs

// 860. Lemonade Change
// https://leetcode.com/problems/lemonade-change/description/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut changes = HashMap::new();
        for mut bill in bills {
            let original = bill;
            while bill > 0 {
                match bill {
                    5 => {
                        bill -= 5;
                    }
                    10 => {
                        if let Some(c) = changes.get_mut(&5)
                            && *c > 0
                        {
                            bill -= 5;
                            *c -= 1;
                        } else {
                            return false;
                        }
                    }
                    15 | 20 => {
                        if let Some(c) = changes.get_mut(&10)
                            && *c > 0
                        {
                            bill -= 10;
                            *c -= 1;
                        } else if let Some(c) = changes.get_mut(&5)
                            && *c > 0
                        {
                            bill -= 5;
                            *c -= 1;
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                }
            }

            *changes.entry(original).or_insert(0) += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let bills = vec![5, 5, 5, 10, 20];
        let result = Solution::lemonade_change(bills);
        assert!(result);
    }

    #[test]
    fn test2() {
        let bills = vec![5, 5, 10, 10, 20];
        let result = Solution::lemonade_change(bills);
        assert!(!result);
    }

    #[test]
    fn test3() {
        let bills = vec![
            5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5,
        ];
        let result = Solution::lemonade_change(bills);
        assert!(result);
    }
}
