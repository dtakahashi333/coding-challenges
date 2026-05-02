// rust/src/leet_coode/easy/fizz_buzz.rs

// 412. Fizz Buzz
// https://leetcode.com/problems/fizz-buzz/description/

pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = Vec::<String>::new();

        for i in 1..=n {
            let mut s = String::new();
            let mut is_fizz = false;
            let mut is_buzz = false;

            if i % 3 == 0 {
                s += "Fizz";
                is_fizz = true;
            }
            if i % 5 == 0 {
                s += "Buzz";
                is_buzz = true;
            }
            if !is_fizz && !is_buzz {
                s += &i.to_string();
            }
            result.push(s);
        }
        result
    }

    pub fn fizz_buzz2(n: i32) -> Vec<String> {
        let mut result = Vec::<String>::new();

        for i in 1..=n {
            let s = match (i % 3, i % 5) {
                (0, 0) => "FizzBuzz".to_string(),
                (0, _) => "Fizz".to_string(),
                (_, 0) => "Buzz".to_string(),
                _ => i.to_string(),
            };
            result.push(s);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 3;
        let result = Solution::fizz_buzz(n);
        assert_eq!(result, ["1", "2", "Fizz"].map(|x| x.to_string()));
    }

    #[test]
    fn test2() {
        let n = 5;
        let result = Solution::fizz_buzz(n);
        assert_eq!(
            result,
            ["1", "2", "Fizz", "4", "Buzz"].map(|x| x.to_string())
        );
    }

    #[test]
    fn test3() {
        let n = 15;
        let result = Solution::fizz_buzz(n);
        assert_eq!(
            result,
            [
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
            .map(|x| x.to_string())
        );
    }

    #[test]
    fn test4() {
        let n = 3;
        let result = Solution::fizz_buzz2(n);
        assert_eq!(result, ["1", "2", "Fizz"].map(|x| x.to_string()));
    }

    #[test]
    fn test5() {
        let n = 5;
        let result = Solution::fizz_buzz2(n);
        assert_eq!(
            result,
            ["1", "2", "Fizz", "4", "Buzz"].map(|x| x.to_string())
        );
    }

    #[test]
    fn test6() {
        let n = 15;
        let result = Solution::fizz_buzz2(n);
        assert_eq!(
            result,
            [
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
            .map(|x| x.to_string())
        );
    }
}
