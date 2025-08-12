struct Solution;

use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut symbols = [-1; 256];
        let mut result: i32 = 0;
        let mut current_result: i32 = 0;
        let mut current_start_index: i32 = 0;

        for (i, c) in s.chars().enumerate() {
            let last_index = symbols[c as usize];
            if last_index >= 0 {
                if current_result > result {
                    result = current_result;
                }
                current_result -= last_index - current_start_index;
                current_start_index = last_index + 1;

                for j in &mut symbols {
                    if *j < last_index && *j >= 0 {
                        *j = -1;
                    }
                }
            } else {
                current_result += 1;
            }
            symbols[c as usize] = i as i32;
        }

        max(result, current_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
