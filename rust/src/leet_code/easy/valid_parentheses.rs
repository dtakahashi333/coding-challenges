// rust/src/leet_code/easy/valid_parentheses.rs

// 20. Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/

#[derive(Debug)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(&self, s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' => {
                    let expected = match c {
                        ')' => '(',
                        ']' => '[',
                        '}' => '{',
                        _ => unreachable!(),
                    };
                    if stack.pop() != Some(expected) {
                        return false;
                    }
                }
                _ => panic!(
                    "Invalid character '{}'. Only '(', ')', '[', ']', '{{', '}}' are allowed.",
                    c
                ),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let parentheses = "()".to_string();
        let s = Solution;
        let result = s.is_valid(parentheses);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let parentheses = "()[]{}".to_string();
        let s = Solution;
        let result = s.is_valid(parentheses);
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let parentheses = "(]".to_string();
        let s = Solution;
        let result = s.is_valid(parentheses);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let parentheses = "([])".to_string();
        let s = Solution;
        let result = s.is_valid(parentheses);
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let parentheses = "([)]".to_string();
        let s = Solution;
        let result = s.is_valid(parentheses);
        assert_eq!(result, false);
    }

    #[test]
    fn test6() {
        let parentheses = "[".to_string();
        let s = Solution;
        let result = s.is_valid(parentheses);
        assert_eq!(result, false);
    }
}
