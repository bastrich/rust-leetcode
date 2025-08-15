struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let bytes = s.as_bytes();
        let mut result = vec![0u8; s.len()];

        let mut k = 0;
        for i in 0..num_rows as usize {
            let mut j = i;
            while j < bytes.len() {
                result[k] = bytes[j];

                j += if (j - i) % (num_rows * 2 - 2) as usize == 0 {
                    (num_rows * 2 - 2) as usize - (2 * i) % (num_rows * 2 - 2) as usize
                } else {
                    2 * i
                };

                k += 1;
            }
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
    }

}
