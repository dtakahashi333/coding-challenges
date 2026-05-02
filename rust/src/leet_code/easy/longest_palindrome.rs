// rust/src/leet_code/easy/longest_palindrome.rs

// 409. Longest Palindrome
// https://leetcode.com/problems/longest-palindrome/description/

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counts = [0; 52];

        for b in s.bytes() {
            if b >= 97 {
                counts[(b - b'a' + 26) as usize] += 1;
            } else {
                counts[(b - b'A') as usize] += 1;
            }
        }

        let mut result = 0;
        let mut has_odd = false;

        for c in counts {
            if c % 2 == 0 {
                result += c;
            } else {
                result += c - 1;
                has_odd = true;
            }
        }
        result + (if has_odd { 1 } else { 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "abccccdd";
        let result = Solution::longest_palindrome(s.into());
        assert_eq!(result, 7);
    }

    #[test]
    fn test2() {
        let s = "a";
        let result = Solution::longest_palindrome(s.into());
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let s = "Za";
        let result = Solution::longest_palindrome(s.into());
        assert_eq!(result, 1);
    }

    #[test]
    fn test4() {
        let s = "civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth";
        let result = Solution::longest_palindrome(s.into());
        assert_eq!(result, 983);
    }
}
