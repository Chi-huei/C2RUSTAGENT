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

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 {
        return head;
    }
    
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    
    let mut length = 0;
    let mut cur = dummy.next.as_ref();
    
    // Calculate total length
    while let Some(node) = cur {
        length += 1;
        cur = node.next.as_ref();
    }
    
    let mut pre = &mut dummy;
    
    for _ in 0..(length / k) {
        let mut group_head = pre.next.take();
        let mut group_nodes = Vec::new();
        
        // Collect k nodes
        for _ in 0..k {
            if let Some(mut node) = group_head {
                group_head = node.next.take();
                group_nodes.push(node);
            }
        }
        
        // Reverse the group
        group_nodes.reverse();
        
        // Get the length before consuming the vector
        let group_len = group_nodes.len();
        
        // Connect reversed group
        for (i, mut node) in group_nodes.into_iter().enumerate() {
            if i == group_len - 1 {
                node.next = group_head.take();
            } else {
                node.next = None;
            }
            pre.next = Some(node);
            pre = pre.next.as_mut().unwrap();
        }
    }
    
    dummy.next
}