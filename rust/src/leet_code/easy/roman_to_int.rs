// rust/src/leet_code/easy/roman_to_int.rs

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn roman_to_int(&self, s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let string_length = chars.len();
        let mut result: i32 = 0;
        let mut i = 0;
        while i < string_length {
            let value = match chars[i] {
                'I' if i + 1 < string_length && chars[i + 1] == 'V' => {
                    i += 2;
                    4
                }
                'I' if i + 1 < string_length && chars[i + 1] == 'X' => {
                    i += 2;
                    9
                }
                'I' => {
                    i += 1;
                    1
                }
                'V' => {
                    i += 1;
                    5
                }
                'X' if i + 1 < string_length && chars[i + 1] == 'L' => {
                    i += 2;
                    40
                }
                'X' if i + 1 < string_length && chars[i + 1] == 'C' => {
                    i += 2;
                    90
                }
                'X' => {
                    i += 1;
                    10
                }
                'L' => {
                    i += 1;
                    50
                }
                'C' if i + 1 < string_length && chars[i + 1] == 'D' => {
                    i += 2;
                    400
                }
                'C' if i + 1 < string_length && chars[i + 1] == 'M' => {
                    i += 2;
                    900
                }
                'C' => {
                    i += 1;
                    100
                }
                'D' => {
                    i += 1;
                    500
                }
                'M' => {
                    i += 1;
                    1000
                }
                _ => {
                    i += 1;
                    0
                }
            };
            result += value;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let roman = String::from("III");
        let s = Solution;
        let result = s.roman_to_int(roman);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let roman = String::from("LVIII");
        let s = Solution;
        let result = s.roman_to_int(roman);
        assert_eq!(result, 58);
    }

    #[test]
    fn test3() {
        let roman = String::from("MCMXCIV");
        let s = Solution;
        let result = s.roman_to_int(roman);
        assert_eq!(result, 1994);
    }
}
