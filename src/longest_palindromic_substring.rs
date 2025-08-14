struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s;
        }

        let bytes = s.as_bytes();
        if s.len() == 2 {
            if bytes[0] == bytes[1] {
                return s;
            } else {
                return s[..1].to_string();
            }
        }

        let mut candidates: Vec<(usize, bool, usize)> = Vec::new();

        if bytes[0] == bytes[1] {
            candidates.push((1, false, 2));
        }

        let mut i = 2;
        while i < bytes.len() {
            for candidate in &mut candidates {
                if !candidate.1 {
                    continue;
                }

                let symmetric_index = candidate.0 as i32 - (candidate.2 / 2) as i32 - 1;
                if symmetric_index >= 0 && bytes[i] == bytes[symmetric_index as usize] {
                    candidate.2 += 2;
                } else {
                    candidate.1 = false;
                }
            }

            if bytes[i] == bytes[i - 1] {
                candidates.push((i, true, 2usize));
            }
            if bytes[i] == bytes[i - 2] {
                candidates.push((i - 1, true, 3usize));
            }

            i += 1;
        }

        candidates
            .iter()
            .max_by_key(|&(_, _, length)| length)
            .map(|&(center_index, _, length)| {
                s[center_index - length / 2..center_index + (length + 1) / 2].to_string()
            })
            .unwrap_or(s[..1].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "aba");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::longest_palindrome("asdhiaushabcdefggfedcbaaaaa".to_string()),
            "abcdefggfedcba"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::longest_palindrome("ccd".to_string()), "cc");
    }

}
