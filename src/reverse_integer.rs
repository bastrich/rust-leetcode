struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut result = 0i32;

        loop {
            let remainder = x % 10;

            result = result.checked_mul(10)
                .and_then(|val| val.checked_add(remainder))
                .unwrap_or_else(|| return 0);

            x = x / 10;
            if x == 0 {
                return result;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::reverse(-2147483412), -2143847412);
    }
}
