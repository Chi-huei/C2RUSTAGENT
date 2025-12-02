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
    if head.is_none() || k <= 1 {
        return head;
    }
    
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut prev_group_end = &mut dummy as *mut Box<ListNode>;
    
    loop {
        // Find the k-th node from current position
        let mut kth_node = unsafe { (*prev_group_end).next.as_ref() };
        for _ in 0..k {
            if kth_node.is_none() {
                return dummy.next;
            }
            kth_node = kth_node.unwrap().next.as_ref();
        }
        
        // Reverse the group
        let mut current = unsafe { (*prev_group_end).next.take() };
        let mut prev: Option<Box<ListNode>> = None;
        
        for _ in 0..k {
            if let Some(mut node) = current {
                current = node.next.take();
                node.next = prev;
                prev = Some(node);
            }
        }
        
        // Connect the reversed group
        let group_start = unsafe { &mut (*prev_group_end).next };
        *group_start = prev;
        
        // Find the new end of the reversed group
        let mut new_prev_group_end = group_start;
        for _ in 0..k-1 {
            if let Some(ref mut node) = new_prev_group_end {
                new_prev_group_end = &mut node.next;
            }
        }
        
        // Connect to the rest of the list
        if let Some(ref mut end_node) = new_prev_group_end {
            end_node.next = current;
        }
        
        // Update prev_group_end for next iteration
        prev_group_end = new_prev_group_end as *mut Option<Box<ListNode>> as *mut Box<ListNode>;
        
        // Check if we can continue
        let mut check_node = unsafe { (*prev_group_end).next.as_ref() };
        for _ in 0..k {
            if check_node.is_none() {
                return dummy.next;
            }
            check_node = check_node.unwrap().next.as_ref();
        }
    }
}