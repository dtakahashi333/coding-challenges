// rust/src/leet_code/easy/find_the_difference.rs

// 389. Find the Difference
// http://leetcode.com/problems/find-the-difference/description/

pub struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut counts = [0; 26];
        for b in s.bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        for b in t.bytes() {
            let index = (b - b'a') as usize;
            counts[index] -= 1;
            if counts[index] < 0 {
                return b as char;
            }
        }
        unreachable!() // More idiomatic than 0 as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("abcd");
        let t = String::from("abcde");
        let result = Solution::find_the_difference(s, t);
        assert_eq!(result, 'e');
    }

    #[test]
    fn test2() {
        let s = String::from("");
        let t = String::from("y");
        let result = Solution::find_the_difference(s, t);
        assert_eq!(result, 'y');
    }
}
