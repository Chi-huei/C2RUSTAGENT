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

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k == 1 {
        return head;
    }
    
    // Count total nodes
    let mut n = 0;
    let mut p = head.as_ref();
    while let Some(node) = p {
        n += 1;
        p = node.next.as_ref();
    }
    
    if n / k == 0 {
        return head;
    }
    
    let mut head = head;
    let mut result = None;
    let mut prev_tail: Option<Box<ListNode>> = None;
    
    for group in 0..(n / k) {
        let mut group_head = head.take();
        
        // Reverse k nodes
        let mut reversed_head = None;
        for _ in 0..k {
            if let Some(mut node) = group_head {
                group_head = node.next.take();
                node.next = reversed_head;
                reversed_head = Some(node);
            }
        }
        
        if group == 0 {
            result = reversed_head.clone();
        }
        
        // Connect with previous group
        if let Some(mut prev) = prev_tail.take() {
            prev.next = reversed_head.clone();
            // Find the new prev_tail
            let mut current = Some(prev);
            while let Some(ref node) = current {
                if node.next.is_none() {
                    break;
                }
                current = current.unwrap().next;
            }
            prev_tail = current;
        }
        
        // Find the tail of current reversed group
        if prev_tail.is_none() {
            let mut current = reversed_head.as_ref();
            while let Some(node) = current {
                if node.next.is_none() {
                    // Create a new node with same value to use as prev_tail
                    prev_tail = Some(Box::new(ListNode::new(node.val)));
                    break;
                }
                current = node.next.as_ref();
            }
        }
        
        head = group_head;
    }
    
    // Connect remaining nodes
    if let Some(mut prev) = prev_tail {
        prev.next = head;
    }
    
    result
}