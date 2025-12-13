// rust/src/leet_code/easy/generate.rs

// 118. Pascal's Triangle
// https://leetcode.com/problems/pascals-triangle/description/

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn generate(&self, num_rows: i32) -> Vec<Vec<i32>> {
        match num_rows {
            0 => vec![],
            1 => vec![vec![1]],
            2 => vec![vec![1], vec![1, 1]],
            _ => {
                let mut triangle = vec![vec![1], vec![1, 1]];
                for i in 1..(num_rows - 1) as usize {
                    let row = &triangle[i];
                    let new_row_size = row.len() + 1;
                    let mut new_row = vec![1; new_row_size];
                    let mut k = 1;
                    for j in 0..row.len() / 2 {
                        let sum = row[j] + row[j + 1];
                        new_row[k] = sum;
                        new_row[new_row_size - 1 - k] = sum;
                        k += 1;
                    }
                    triangle.push(new_row);
                }
                triangle
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let num_rows = 5;
        let s = Solution;
        let result = s.generate(num_rows);
        assert_eq!(
            result,
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ]
        );
    }

    #[test]
    fn test2() {
        let num_rows = 1;
        let s = Solution;
        let result = s.generate(num_rows);
        assert_eq!(result, vec![vec![1]]);
    }
}
