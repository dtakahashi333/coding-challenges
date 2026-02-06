// rust/src/leet_code/easy/first_bad_version.rs

// 278. First Bad Version
// https://leetcode.com/problems/first-bad-version/description/

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

#[derive(Debug)]
pub struct Solution {
    versions: Vec<bool>,
}

impl Solution {
    pub fn new(n: i32, bad: i32) -> Self {
        let mut versions = Vec::with_capacity(n as usize);

        for i in 0..n {
            versions.push(i >= bad - 1);
        }

        Self { versions }
    }

    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        self.versions[(version - 1) as usize]
    }
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut start = 1;
        let mut end = n;
        let mut bad = end;

        while start <= end {
            let version: i32 = start + (end - start) / 2;
            if self.isBadVersion(version) {
                bad = version;
                end = version - 1;
            } else {
                start = version + 1;
            }
        }

        bad
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 5;
        let bad = 4;
        let s = Solution::new(n, bad);
        let result = s.first_bad_version(n);
        assert_eq!(result, bad);
    }

    #[test]
    fn test2() {
        let n = 1;
        let bad = 1;
        let s = Solution::new(n, bad);
        let result = s.first_bad_version(n);
        assert_eq!(result, bad);
    }

    #[test]
    fn test3() {
        let n = 3;
        let bad = 1;
        let s = Solution::new(n, bad);
        let result = s.first_bad_version(n);
        assert_eq!(result, bad);
    }
}
