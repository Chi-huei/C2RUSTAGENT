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
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }
    
    let mut new_head = Box::new(ListNode::new(0));
    new_head.next = head;
    
    let mut pre_node = &mut new_head as *mut Box<ListNode>;
    
    loop {
        let mut expendable = 0;
        let mut check_node = unsafe { (*pre_node).next.as_ref() };
        
        // Check if we have k nodes remaining
        while expendable < k {
            if check_node.is_none() {
                return new_head.next;
            }
            check_node = check_node.unwrap().next.as_ref();
            expendable += 1;
        }
        
        // Reverse k nodes
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = unsafe { (*pre_node).next.take() };
        let mut count = 0;
        
        while count < k && curr.is_some() {
            let mut node = curr.unwrap();
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
            count += 1;
        }
        
        // Connect the reversed group
        unsafe {
            (*pre_node).next = prev;
            
            // Find the tail of the reversed group (original head)
            let mut tail = (*pre_node).next.as_mut();
            while tail.as_ref().unwrap().next.is_some() {
                tail = tail.unwrap().next.as_mut();
            }
            
            // Connect to remaining nodes
            tail.as_mut().unwrap().next = curr;
            
            // Update pre_node for next iteration
            pre_node = tail.unwrap() as *mut Box<ListNode>;
        }
    }
}