#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

fn has_k_nodes(head: &Option<Box<ListNode>>, k: i32) -> bool {
    if head.is_none() {
        return false;
    }
    
    let mut p = head.as_ref();
    let mut remaining = k;
    
    while let Some(node) = p {
        if remaining <= 0 {
            break;
        }
        remaining -= 1;
        p = node.next.as_ref();
    }
    
    remaining <= 0
}

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 {
        return head;
    }
    
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    
    let mut prev_group_end = &mut dummy as *mut Box<ListNode>;
    
    unsafe {
        while has_k_nodes(&(*prev_group_end).next, k) {
            let mut current = (*prev_group_end).next.take();
            let group_start = current.as_mut().unwrap();
            
            // Reverse k-1 connections within the group
            for _ in 0..k-1 {
                let mut next_node = group_start.next.take();
                group_start.next = next_node.as_mut().unwrap().next.take();
                next_node.as_mut().unwrap().next = (*prev_group_end).next.take();
                (*prev_group_end).next = next_node;
            }
            
            // Connect the previous group to the current reversed group
            (*prev_group_end).next = current;
            
            // Move prev_group_end to the end of current group
            while (*prev_group_end).next.is_some() {
                prev_group_end = (*prev_group_end).next.as_mut().unwrap();
            }
        }
    }
    
    dummy.next
}