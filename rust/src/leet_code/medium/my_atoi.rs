// rust/src/leet_code/medium/my_atoi.rs

// 8. String to Integer (atoi)
// https://leetcode.com/problems/string-to-integer-atoi/description/

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut i = 0;

        // Skip leading spaces
        while i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }

        let mut sign = 1;

        if i < bytes.len() {
            match bytes[i] {
                b'+' => i += 1,
                b'-' => {
                    sign = -1;
                    i += 1;
                }
                _ => {}
            }
        }

        Self::parse(bytes, i, sign, 0)
    }

    fn parse(bytes: &[u8], i: usize, sign: i32, acc: i32) -> i32 {
        if i >= bytes.len() || !bytes[i].is_ascii_digit() {
            return acc * sign;
        }

        let digit = (bytes[i] - b'0') as i32;

        // Overflow check before acc * 10 + digit
        if acc > (i32::MAX - digit) / 10 {
            return if sign == 1 { i32::MAX } else { i32::MIN };
        }

        Self::parse(bytes, i + 1, sign, acc * 10 + digit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("42");
        let result = Solution::my_atoi(s);
        assert_eq!(result, 42);
    }

    #[test]
    fn test2() {
        let s = String::from("-42");
        let result = Solution::my_atoi(s);
        assert_eq!(result, -42);
    }

    #[test]
    fn test3() {
        let s = String::from("1337c0d3");
        let result = Solution::my_atoi(s);
        assert_eq!(result, 1337);
    }

    #[test]
    fn test4() {
        let s = String::from("0-1");
        let result = Solution::my_atoi(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn test5() {
        let s = String::from("words and 987");
        let result = Solution::my_atoi(s);
        assert_eq!(result, 0);
    }
}
