#![allow(dead_code)]

/*

You are given two non-empty linked lists representing two non-negative integers.
The digits are stored in reverse order, and each of their nodes contains a single digit.
Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

*/

// Definition for singly-linked list.
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

pub fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() {
        let mut new_node = Box::new(ListNode::new(val));
        new_node.next = current;
        current = Some(new_node);
    }
    current
}

pub fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = list;
    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }
    result
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut a = l1;
    let mut b = l2;

    let mut dummy_head = Some(Box::new(ListNode::new(0)));
    let mut current = &mut dummy_head;

    while a.is_some() || b.is_some() || carry != 0 {
        let sum =
            carry + a.as_ref().map_or(0, |node| node.val) + b.as_ref().map_or(0, |node| node.val);

        carry = sum / 10;

        if let Some(curr_node) = current {
            curr_node.next = Some(Box::new(ListNode::new(sum % 10)));
            current = &mut curr_node.next;
        }

        a = a.and_then(|node| node.next);
        b = b.and_then(|node| node.next);
    }

    dummy_head.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers_case1() {
        let l1 = to_linked_list(vec![2, 4, 3]);
        let l2 = to_linked_list(vec![5, 6, 4]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(to_vec(result), vec![7, 0, 8]);
    }

    #[test]
    fn test_add_two_numbers_case2() {
        let l1 = to_linked_list(vec![0]);
        let l2 = to_linked_list(vec![0]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(to_vec(result), vec![0]);
    }

    #[test]
    fn test_add_two_numbers_case3() {
        let l1 = to_linked_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = to_linked_list(vec![9, 9, 9, 9]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(to_vec(result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn test_add_two_numbers_different_lengths() {
        let l1 = to_linked_list(vec![1, 8]);
        let l2 = to_linked_list(vec![0]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(to_vec(result), vec![1, 8]);
    }

    #[test]
    fn test_add_two_numbers_with_carry() {
        let l1 = to_linked_list(vec![9, 9]);
        let l2 = to_linked_list(vec![1]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(to_vec(result), vec![0, 0, 1]);
    }
}
