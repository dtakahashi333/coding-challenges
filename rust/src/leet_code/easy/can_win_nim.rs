// rust/src/leet_code/easy/can_win_nim.rs

// 292. Nim Game
// https://leetcode.com/problems/nim-game/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 4;
        let result = Solution::can_win_nim(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test2() {
        let n = 1;
        let result = Solution::can_win_nim(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let n = 2;
        let result = Solution::can_win_nim(n);
        assert_eq!(result, true);
    }
}
