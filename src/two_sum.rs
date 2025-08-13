struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted_nums: Vec<(usize, i32)> =  nums.iter()
            .enumerate()
            .map(|(i, &v)| (i, v))
            .collect();
        sorted_nums.sort_by_key(|pair| pair.1);
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut sum = sorted_nums[i].1 + sorted_nums[j].1;

        while sum != target {
            if sum > target {
                j -= 1;
            } else {
                i += 1;
            }
            sum = sorted_nums[i].1 + sorted_nums[j].1;
        }

        vec![sorted_nums[i].0 as i32, sorted_nums[j].0 as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
