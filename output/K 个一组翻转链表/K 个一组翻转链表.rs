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
    
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut prev_group_end = &mut dummy as *mut Box<ListNode>;
    
    loop {
        // Find the start and end of the current group
        let mut group_start = unsafe { (*prev_group_end).next.take() };
        if group_start.is_none() {
            break;
        }
        
        let mut current = &mut group_start;
        let mut count = 0;
        
        // Check if we have k nodes in this group
        while count < k {
            if current.is_none() {
                // Not enough nodes, restore the connection and return
                unsafe { (*prev_group_end).next = group_start };
                return dummy.next;
            }
            if count == k - 1 {
                break;
            }
            current = &mut current.as_mut().unwrap().next;
            count += 1;
        }
        
        // Split the group from the rest of the list
        let next_group_start = current.as_mut().unwrap().next.take();
        
        // Reverse the current group
        let reversed_group = reverse_list(group_start);
        
        // Connect the reversed group back to the list
        unsafe {
            (*prev_group_end).next = reversed_group;
            
            // Find the new end of the reversed group (which was the original start)
            let mut new_end = &mut (*prev_group_end).next;
            while new_end.as_ref().unwrap().next.is_some() {
                new_end = &mut new_end.as_mut().unwrap().next;
            }
            
            // Connect to the next group
            new_end.as_mut().unwrap().next = next_group_start;
            
            // Update prev_group_end for the next iteration
            prev_group_end = new_end.as_mut().unwrap() as *mut Box<ListNode>;
        }
    }
    
    dummy.next
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;
    
    while let Some(mut node) = current {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        current = next;
    }
    
    prev
}