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
            val
        }
    }
}

fn reverse(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(ref mut head_node) = head {
        if head_node.next.is_none() {
            return None;
        }
        
        let mut curfront = head_node.next.take();
        if let Some(ref mut curfront_node) = curfront {
            let mut cur = curfront_node.next.take();
            
            while let Some(mut cur_node) = cur {
                let curnext = cur_node.next.take();
                cur_node.next = curfront;
                curfront = Some(cur_node);
                cur = curnext;
            }
        }
        
        head_node.next = curfront;
        head.clone()
    } else {
        None
    }
}

fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    
    let mut dummy_head = Box::new(ListNode::new(0));
    dummy_head.next = head;
    let mut dummy_head = Some(dummy_head);
    
    let mut i = 1;
    let mut curfront = &mut dummy_head;
    
    loop {
        // Count k nodes
        let mut cur = curfront.as_ref().unwrap().next.as_ref();
        let mut count = 0;
        
        while let Some(node) = cur {
            count += 1;
            if count == k {
                break;
            }
            cur = node.next.as_ref();
        }
        
        if count < k {
            break;
        }
        
        // Find the k-th node and break the connection
        let mut cur_ref = &mut curfront.as_mut().unwrap().next;
        for _ in 0..k-1 {
            cur_ref = &mut cur_ref.as_mut().unwrap().next;
        }
        
        let curnext = cur_ref.as_mut().unwrap().next.take();
        
        // Reverse the k nodes
        let tail = curfront.as_ref().unwrap().next.as_ref().unwrap().val;
        reverse(curfront);
        
        // Find the new tail (original head of the k-group)
        let mut new_tail = &mut curfront.as_mut().unwrap().next;
        while new_tail.as_ref().unwrap().next.is_some() {
            new_tail = &mut new_tail.as_mut().unwrap().next;
        }
        
        // Connect to the rest
        new_tail.as_mut().unwrap().next = curnext;
        
        // Move curfront to the new tail
        while curfront.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
            curfront = unsafe { 
                std::mem::transmute(&mut curfront.as_mut().unwrap().next)
            };
        }
    }
    
    dummy_head.unwrap().next
}