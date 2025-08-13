struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_internal(l1, l2, 0)
    }

    pub fn add_two_numbers_internal(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        addition: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && addition == 0 {
            return None;
        }
        if l1.is_none() && l2.is_none() {
            return Some(Box::new(ListNode::new(addition)));
        }
        if l1.is_none() {
            return Self::add_two_numbers_internal(l2, Some(Box::new(ListNode::new(addition))), 0);
        }
        if l2.is_none() {
            return Self::add_two_numbers_internal(l1, Some(Box::new(ListNode::new(addition))), 0);
        }

        let l1 = l1.unwrap();
        let val1 = l1.val;
        let next1 = l1.next;

        let l2 = l2.unwrap();
        let val2 = l2.val;
        let next2 = l2.next;

        let result = val1 + val2 + addition;

        let mut list_node = ListNode::new(result % 10);
        list_node.next = Self::add_two_numbers_internal(
            next1,
            next2,
            if result > 9 { 1 } else { 0 }
        );

        Some(Box::new(list_node))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(nums: &[i32]) -> Option<Box<ListNode>> {
        if nums.len() == 1 {
            Some(Box::new(ListNode::new(nums[0])))
        } else {
            Some(Box::new(ListNode {
                val: nums[0],
                next: create_list(&nums[1..]),
            }))
        }
    }

    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_two_numbers(create_list(&[2, 4, 3]), create_list(&[5, 6, 4])),
            create_list(&[7, 0, 8])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::add_two_numbers(create_list(&[0]), create_list(&[0])),
            create_list(&[0])
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::add_two_numbers(create_list(&[9,9,9,9,9,9,9]), create_list(&[9,9,9,9])),
            create_list(&[8,9,9,9,0,0,0,1])
        );
    }
}
