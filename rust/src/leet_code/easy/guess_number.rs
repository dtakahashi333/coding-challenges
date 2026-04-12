// rust/src/leet_code/easy/guess_number.rs

// 374. Guess Number Higher or Lower
// https://leetcode.com/problems/guess-number-higher-or-lower/description/

static mut PICK: i64 = 0;

// dummy definition
#[allow(dead_code)]
unsafe fn guess(num: i32) -> i32 {
    let num: i64 = num as i64;
    if num > unsafe { PICK } {
        return -1;
    } else if num < unsafe { PICK } {
        return 1;
    }
    0
}

/**
 * Forward declaration of guess API.
 * @param  num  your guess
 * @return      -1 if num is higher than the picked number
 *              1 if num is lower than the picked number
 *              otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */
pub struct Solution;

impl Solution {
    #[allow(non_snake_case, clippy::missing_safety_doc)]
    pub unsafe fn guessNumber(n: i32) -> i32 {
        let mut lower = 1_i64;
        let mut upper: i64 = n as i64;

        while lower <= upper {
            let m = lower + (upper - lower) / 2;
            let res = unsafe { guess(m as i32) };
            if res == -1 {
                upper = m - 1;
            } else if res == 1 {
                lower = m + 1;
            } else {
                return m as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        unsafe {
            PICK = 6;
            let n = 10;
            let pick: i32 = PICK as i32;
            let result = Solution::guessNumber(n);
            assert_eq!(result, pick);
        }
    }

    #[test]
    fn test2() {
        unsafe {
            PICK = 1;
            let n = 1;
            let pick: i32 = PICK as i32;
            let result = Solution::guessNumber(n);
            assert_eq!(result, pick);
        }
    }

    #[test]
    fn test3() {
        unsafe {
            PICK = 1;
            let n = 2;
            let pick: i32 = PICK as i32;
            let result = Solution::guessNumber(n);
            assert_eq!(result, pick);
        }
    }

    #[test]
    fn test4() {
        unsafe {
            PICK = 1702766719;
            let n = 2126753390;
            let pick: i32 = PICK as i32;
            let result = Solution::guessNumber(n);
            assert_eq!(result, pick);
        }
    }
}
