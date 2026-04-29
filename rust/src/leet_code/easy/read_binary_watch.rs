// rust/src/leet_code/easy/read_binary_watch.rs

// 401. Binary Watch
// https://leetcode.com/problems/binary-watch/description/

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        if turned_on == 0 {
            return vec![String::from("0:00")];
        }
        let mut result = HashSet::<String>::new();
        let mut path = Vec::<i32>::new();
        Self::backtrack(0, &mut path, turned_on, &mut result);
        result.into_iter().collect()
    }

    fn backtrack(start: i32, path: &mut Vec<i32>, turned_on: i32, result: &mut HashSet<String>) {
        if turned_on == 0 {
            if let Some(time_string) = Self::to_time_string(path) {
                result.insert(time_string);
            }
            return;
        }
        for i in start..10 {
            path.push(i);
            Self::backtrack(i + 1, path, turned_on - 1, result);
            path.pop();
        }
    }

    fn to_time_string(path: &Vec<i32>) -> Option<String> {
        let mut hour = 0;
        let mut minute = 0;
        for &pos in path {
            if pos < 4 {
                // hour
                hour += 1 << (3 - pos)
            } else {
                // minute
                minute += 1 << (9 - pos)
            }
        }
        if hour < 12 && minute < 60 {
            Some(format!("{}:{:02}", hour, minute))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let turned_on = 1;
        let mut result = Solution::read_binary_watch(turned_on);
        let mut expected: Vec<String> = vec![
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ]
        .into_iter()
        .map(String::from) // or .map(|s| s.to_string())
        .collect();
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let turned_on = 9;
        let result = Solution::read_binary_watch(turned_on);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test3() {
        let turned_on = 2;
        let mut result = Solution::read_binary_watch(turned_on);
        let mut expected: Vec<String> = vec![
            "0:03", "0:05", "0:06", "0:09", "0:10", "0:12", "0:17", "0:18", "0:20", "0:24", "0:33",
            "0:34", "0:36", "0:40", "0:48", "1:01", "1:02", "1:04", "1:08", "1:16", "1:32", "2:01",
            "2:02", "2:04", "2:08", "2:16", "2:32", "3:00", "4:01", "4:02", "4:04", "4:08", "4:16",
            "4:32", "5:00", "6:00", "8:01", "8:02", "8:04", "8:08", "8:16", "8:32", "9:00",
            "10:00",
        ]
        .into_iter()
        .map(String::from) // or .map(|s| s.to_string())
        .collect();
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }
}
