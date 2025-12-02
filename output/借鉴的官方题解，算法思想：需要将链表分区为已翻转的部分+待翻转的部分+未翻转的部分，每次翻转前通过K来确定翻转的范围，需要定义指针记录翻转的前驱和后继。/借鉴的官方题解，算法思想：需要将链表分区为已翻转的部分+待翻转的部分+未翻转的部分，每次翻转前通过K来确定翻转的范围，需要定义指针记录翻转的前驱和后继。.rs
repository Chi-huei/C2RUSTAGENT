use std::ptr;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    let mut curr = head;
    
    while let Some(mut node) = curr {
        let next = node.next.take();
        node.next = pre;
        pre = Some(node);
        curr = next;
    }
    
    pre
}

fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    
    let mut pre = &mut *dummy as *mut ListNode;
    let mut end = &mut *dummy as *mut ListNode;
    
    unsafe {
        while (*end).next.is_some() {
            for _ in 0..k {
                if let Some(ref mut next_node) = (*end).next {
                    end = &mut **next_node as *mut ListNode;
                } else {
                    end = ptr::null_mut();
                    break;
                }
            }
            
            if end.is_null() {
                break;
            }
            
            let start = (*pre).next.take();
            let next = (*end).next.take();
            
            let reversed = reverse(start);
            (*pre).next = reversed;
            
            let mut current = &mut (*pre).next;
            while let Some(ref mut node) = current {
                if node.next.is_none() {
                    node.next = next;
                    pre = &mut **node as *mut ListNode;
                    break;
                }
                current = &mut node.next;
            }
            
            end = pre;
        }
    }
    
    dummy.next
}