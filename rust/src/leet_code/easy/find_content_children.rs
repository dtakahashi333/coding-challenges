// rust/src/leet_code/easy/find_content_children.rs

// 455. Assign Cookies
// https://leetcode.com/problems/assign-cookies/description/

pub struct Solution;

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort_unstable_by(|a, b| b.cmp(a));
        s.sort_unstable_by(|a, b| b.cmp(a));

        let mut i = 0;
        let mut j = 0;
        while i < g.len() && j < s.len() {
            if s[j] >= g[i] {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }

        j as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let g = vec![1, 2, 3];
        let s = vec![1, 1];
        let result = Solution::find_content_children(g, s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let g = vec![1, 2];
        let s = vec![1, 2, 3];
        let result = Solution::find_content_children(g, s);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let g = vec![10, 9, 8, 7];
        let s = vec![5, 6, 7, 8];
        let result = Solution::find_content_children(g, s);
        assert_eq!(result, 2);
    }
}
