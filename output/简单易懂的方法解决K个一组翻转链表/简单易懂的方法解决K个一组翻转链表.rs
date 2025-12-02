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
        let mut kth_node = unsafe { &mut *prev_group_end };
        for _ in 0..k {
            if kth_node.next.is_none() {
                return dummy.next;
            }
            kth_node = kth_node.next.as_mut().unwrap();
        }
        
        let next_group_start = kth_node.next.take();
        
        // Reverse the current group
        let group_start = unsafe { (*prev_group_end).next.take() };
        let (new_group_start, new_group_end) = reverse_list(group_start);
        
        // Connect the reversed group
        unsafe {
            (*prev_group_end).next = new_group_start;
            (*new_group_end).next = next_group_start;
        }
        
        // Move prev_group_end to the end of current group
        prev_group_end = new_group_end;
    }
}

fn reverse_list(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, *mut Box<ListNode>) {
    let mut prev = None;
    let mut current = head;
    let mut tail_ref: *mut Box<ListNode> = std::ptr::null_mut();
    
    while let Some(mut node) = current {
        if tail_ref.is_null() {
            tail_ref = &mut node as *mut Box<ListNode>;
        }
        current = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    
    (prev, tail_ref)
}