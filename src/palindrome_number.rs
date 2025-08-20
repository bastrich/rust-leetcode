struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let string_representation = x.to_string();
        let bytes = string_representation.as_bytes();
        for i in 0..bytes.len() / 2 {
            if bytes[i] != bytes[bytes.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::is_palindrome(1001), true);
    }
}
