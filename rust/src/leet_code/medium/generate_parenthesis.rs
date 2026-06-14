// rust/src/leet_code/medium/generate_parenthesis.rs

// 22. Generate Parentheses
// https://leetcode.com/problems/generate-parentheses/description/

pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::<String>::new();
        Self::generate_parenthesis_helper(n * 2 - 1, String::from("("), &mut result, [n - 1, n]);
        result
    }

    fn generate_parenthesis_helper(
        n: i32,
        paren: String,
        result: &mut Vec<String>,
        paren_count: [i32; 2],
    ) {
        if n == 0 {
            result.push(paren.clone());
            return;
        }
        if paren_count[0] > 0 {
            let paren1 = paren.clone() + "(";
            Self::generate_parenthesis_helper(
                n - 1,
                paren1,
                result,
                [paren_count[0] - 1, paren_count[1]],
            );
        }
        if paren_count[1] > 0 && paren_count[0] < paren_count[1] {
            let paren2 = paren.clone() + ")";
            Self::generate_parenthesis_helper(
                n - 1,
                paren2,
                result,
                [paren_count[0], paren_count[1] - 1],
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 3;
        let mut result = Solution::generate_parenthesis(n);
        result.sort();
        let mut expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let n = 1;
        let mut result = Solution::generate_parenthesis(n);
        result.sort();
        let mut expected = vec!["()"];
        expected.sort();
        assert_eq!(result, expected);
    }
}
