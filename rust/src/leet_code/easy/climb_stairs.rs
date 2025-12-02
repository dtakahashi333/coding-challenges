// rust/src/leet_code/easy/climb_stairs.rs

// 70. Climbing Stairs
// https://leetcode.com/problems/climbing-stairs/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn climb_stairs(&self, n: i32) -> i32 {
        let mut dp = vec![-1; (n + 1) as usize];
        self.climb_stairs_helper(n, &mut dp)
    }

    fn climb_stairs_helper(&self, n: i32, dp: &mut Vec<i32>) -> i32 {
        if n == 0 {
            return 1;
        }
        if dp[n as usize] > 0 {
            return dp[n as usize];
        }
        let mut result = 0;
        // 2 steps
        if n >= 2 {
            result += self.climb_stairs_helper(n - 2, dp);
        }
        // 1 step
        if n >= 1 {
            result += self.climb_stairs_helper(n - 1, dp);
        }
        dp[n as usize] = result;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 2;
        let s = Solution;
        let result = s.climb_stairs(n);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let n = 3;
        let s = Solution;
        let result = s.climb_stairs(n);
        assert_eq!(result, 3);
    }
}
