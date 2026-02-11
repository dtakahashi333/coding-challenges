// rust/src/leet_code/easy/num_array.rs

// 303. Range Sum Query - Immutable
// https://leetcode.com/problems/range-sum-query-immutable/description/

#[derive(Debug)]
pub struct NumArray {
    prefix: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut prefix = Vec::new();
        prefix.push(0);

        for n in nums {
            if let Some(&prev) = prefix.last() {
                prefix.push(prev + n);
            }
        }

        Self { prefix }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix[(right + 1) as usize] - self.prefix[left as usize]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let obj = NumArray::new(nums);
        let ret_1: i32 = obj.sum_range(0, 2);
        let ret_2: i32 = obj.sum_range(2, 5);
        let ret_3: i32 = obj.sum_range(0, 5);
        assert_eq!(vec![ret_1, ret_2, ret_3], vec![1, -1, -3]);
    }
}
