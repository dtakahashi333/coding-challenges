// rust/src/dsa/divide_and_conquer/is_winning_hand.rs

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_winning_hand(nums: Vec<i32>) -> bool {
        if nums.len() != 14 {
            return false;
        }

        let mut map = HashMap::<i32, i32>::new();
        for key in nums {
            *map.entry(key).or_default() += 1;
        }

        let keys: Vec<i32> = map.keys().copied().collect();

        // Step 1: Try choosing each possible tile as the "Eye" (Pair)
        for key in keys {
            if let Some(value) = map.get_mut(&key)
                && *value >= 2
            {
                *value -= 2;

                if Self::is_winning_hand_helper(&mut map) {
                    return true;
                }

                // Backtrack
                if let Some(v) = map.get_mut(&key) {
                    *v += 2;
                }
            }
        }

        false
    }

    fn is_winning_hand_helper(map: &mut HashMap<i32, i32>) -> bool {
        // FIX: Explicitly find the TRUE minimum key remaining with a count > 0
        let min = match map.iter().filter(|(_, v)| **v > 0).map(|(&k, _)| k).min() {
            Some(k) => k,
            // If no tiles are left with a count > 0, we successfully cleared the hand!
            None => return true,
        };

        // ----- 1. Try forming a Triple (Pong) -----
        if let Some(v) = map.get_mut(&min)
            && *v >= 3
        {
            *v -= 3;

            if Self::is_winning_hand_helper(map) {
                return true;
            }

            // Backtrack
            if let Some(v) = map.get_mut(&min) {
                *v += 3;
            }
        }

        // ----- 2. Try forming a Straight (Chow: min, min+1, min+2) -----
        let ok = {
            let a = map.get(&min).copied().unwrap_or(0);
            let b = map.get(&(min + 1)).copied().unwrap_or(0);
            let c = map.get(&(min + 2)).copied().unwrap_or(0);

            a >= 1 && b >= 1 && c >= 1
        };

        if ok {
            *map.get_mut(&min).unwrap() -= 1;
            *map.get_mut(&(min + 1)).unwrap() -= 1;
            *map.get_mut(&(min + 2)).unwrap() -= 1;

            if Self::is_winning_hand_helper(map) {
                return true;
            }

            // Backtrack
            *map.get_mut(&min).unwrap() += 1;
            *map.get_mut(&(min + 1)).unwrap() += 1;
            *map.get_mut(&(min + 2)).unwrap() += 1;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 3, 4, 5, 5, 5, 5, 5, 5, 7, 8, 9];
        let result = Solution::is_winning_hand(nums);
        assert!(result);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7, 8, 9, 9];
        let result = Solution::is_winning_hand(nums);
        assert!(!result);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 2, 3, 3, 4, 5, 6, 6, 7, 7, 8, 8, 9, 9];
        let result = Solution::is_winning_hand(nums);
        assert!(result);
    }
}
