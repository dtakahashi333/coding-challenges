// rust/src/leet_code/easy/is_happy.rs

// 202. Happy Number
// https://leetcode.com/problems/happy-number/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(n);

        let mut current = n;
        loop {
            let mut sum = 0;
            while current > 0 {
                let digit = current % 10;
                sum += digit * digit;
                current /= 10;
            }
            
            if sum == 1 {
                return true;
            }
            if set.contains(&sum) {
                return false;
            }
            set.insert(sum);
            
            current = sum;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 19;
        let result = Solution::is_happy(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let n = 2;
        let result = Solution::is_happy(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let n = 7;
        let result = Solution::is_happy(n);
        assert_eq!(result, true);
    }
}
