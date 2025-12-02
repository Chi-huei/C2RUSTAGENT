use std::boxed::Box;

#[derive(Debug, PartialEq, Eq)]
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

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    
    let mut cur = head;
    let mut prev = None;
    
    while let Some(mut node) = cur {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        cur = next;
    }
    
    prev
}

fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    
    // Check if we have at least k nodes
    let mut cur = &head;
    for _ in 0..k {
        if cur.is_none() {
            return head;
        }
        cur = &cur.as_ref().unwrap().next;
    }
    
    // Split the list at position k
    let mut current = head;
    let mut group_head = None;
    let mut group_tail = None;
    
    // Extract first k nodes
    for i in 0..k {
        if let Some(mut node) = current {
            current = node.next.take();
            if i == 0 {
                group_tail = Some(&mut node as *mut Box<ListNode>);
            }
            node.next = group_head;
            group_head = Some(node);
        }
    }
    
    // Reverse the remaining list recursively
    let rest_reversed = reverse_k_group(current, k);
    
    // Connect the reversed group with the rest
    if let Some(tail_ptr) = group_tail {
        unsafe {
            (*tail_ptr).next = rest_reversed;
        }
    }
    
    group_head
}