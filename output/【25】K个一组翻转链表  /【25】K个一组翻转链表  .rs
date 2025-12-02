use std::boxed::Box;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

fn reverse(mut head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre: Option<Box<ListNode>> = None;
    
    while head != tail {
        if let Some(mut node) = head {
            let next = node.next.take();
            node.next = pre;
            pre = Some(node);
            head = next;
        } else {
            break;
        }
    }
    pre
}

fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }
    
    let mut tail = head.as_ref();
    for _ in 0..k {
        if tail.is_none() {
            return head;
        }
        tail = tail.unwrap().next.as_ref();
    }
    
    let tail_clone = tail.cloned();
    let mut new_head = reverse(head, tail_clone.clone());
    
    if let Some(ref mut node) = new_head {
        let mut current = node;
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }
        current.next = reverse_k_group(tail_clone, k);
    }
    
    new_head
}