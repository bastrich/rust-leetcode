struct Solution;

enum Stage {
    Whitespaces,
    Zeroes,
    Numbers,
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result = 0i32;
        let mut sign = 1;

        let mut stage = Stage::Whitespaces;
        for c in s.chars() {
            match stage {
                Stage::Whitespaces => {
                    if c == ' ' {
                        continue;
                    } else if c == '-' {
                        sign = -1;
                        stage = Stage::Zeroes;
                    } else if c == '+' {
                        stage = Stage::Zeroes;
                    } else if c == '0' {
                        stage = Stage::Zeroes;
                    } else if c.is_numeric() {
                        stage = Stage::Numbers;
                        result = sign * c.to_digit(10).unwrap() as i32;
                    } else {
                        return 0;
                    }
                }
                Stage::Zeroes => {
                    if c == '0' {
                        continue;
                    } else if c.is_numeric() {
                        stage = Stage::Numbers;
                        result = sign * c.to_digit(10).unwrap() as i32;
                    } else {
                        return 0;
                    }
                }
                Stage::Numbers => {
                    if c.is_numeric() {
                        stage = Stage::Numbers;
                        result = result
                            .saturating_mul(10i32)
                            .saturating_add(sign * c.to_digit(10).unwrap() as i32);
                    } else {
                        break;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::my_atoi("   -042".to_string()), -42);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }
}
