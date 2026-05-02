// rust/src/leet_code/easy/to_hex.rs

// 405. Convert a Number to Hexadecimal
// https://leetcode.com/problems/convert-a-number-to-hexadecimal/description/

pub struct Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".into();
        }
        let base = 2u32;
        let mut num = num;
        let mut nibble = 0;
        let mut result = String::new();
        for _ in 0..8 {
            for j in 1..=4 {
                if num == 0 {
                    break;
                }
                // Get the least bit.
                let bit = (num & 1) as u32;
                nibble += base.pow(j - 1) * bit;
                if j % 4 == 0 {
                    result = format!("{:x}{}", nibble, result);
                    nibble = 0;
                }
                num >>= 1;
            }
        }
        if nibble != 0 {
            result = format!("{:x}{}", nibble, result);
        }
        result
    }

    // Gen AI solution
    pub fn to_hex2(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        // A lookup table for hex characters
        let hex_chars = b"0123456789abcdef";

        let mut num = num as u32;
        let mut result = String::new();

        while num != 0 {
            let nibble = (num & 0xf) as usize;

            result.insert(0, hex_chars[nibble] as char);

            num >>= 4;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let num = 26;
        let result = Solution::to_hex(num);
        assert_eq!(result, "1a");
    }

    #[test]
    fn test2() {
        let num = -1;
        let result = Solution::to_hex(num);
        assert_eq!(result, "ffffffff");
    }

    #[test]
    fn test3() {
        let num = 0;
        let result = Solution::to_hex(num);
        assert_eq!(result, "0");
    }

    #[test]
    fn test4() {
        let num = 26;
        let result = Solution::to_hex2(num);
        assert_eq!(result, "1a");
    }

    #[test]
    fn test5() {
        let num = -1;
        let result = Solution::to_hex2(num);
        assert_eq!(result, "ffffffff");
    }

    #[test]
    fn test6() {
        let num = 0;
        let result = Solution::to_hex2(num);
        assert_eq!(result, "0");
    }
}
