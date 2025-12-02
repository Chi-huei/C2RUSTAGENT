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
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    
    let mut k_pre = &mut dummy as *mut Box<ListNode>;
    let mut current = unsafe { (*k_pre).next.as_mut() };
    
    while current.is_some() {
        // Check if we have k nodes remaining
        let mut count = 0;
        let mut temp = current;
        while temp.is_some() && count < k {
            temp = temp.unwrap().next.as_mut();
            count += 1;
        }
        
        if count < k {
            break;
        }
        
        // Reverse k nodes
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = unsafe { (*k_pre).next.take() };
        
        for _ in 0..k {
            if let Some(mut node) = curr {
                let next = node.next.take();
                node.next = prev;
                prev = Some(node);
                curr = next;
            }
        }
        
        // Connect the reversed group
        unsafe {
            (*k_pre).next = prev;
            
            // Find the end of the reversed group
            let mut end = (*k_pre).next.as_mut().unwrap();
            for _ in 1..k {
                end = end.next.as_mut().unwrap();
            }
            
            end.next = curr;
            k_pre = end as *mut Box<ListNode>;
        }
        
        current = unsafe { (*k_pre).next.as_mut() };
    }
    
    dummy.next
}