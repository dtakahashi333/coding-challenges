// rust/src/leet_code/easy/reverse_string.rs

// 344. Reverse String
// https://leetcode.com/problems/reverse-string/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        for i in 0..len / 2 {
            s.swap(i, len - 1 - i);
        }
    }

    // More Idiomatic (Two-Pointer Style) by ChatGPT
    pub fn reverse_string2(s: &mut Vec<char>) {
        let mut left = 0;
        let mut right = s.len().saturating_sub(1);

        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn test2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }

    #[test]
    fn test3() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string2(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn test4() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string2(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
