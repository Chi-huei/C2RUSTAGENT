use std::boxed::Box;

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

fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }

    let mut dummy = Box::new(ListNode::new(0));
    let mut current = head;
    
    // Count total length
    let mut cnt = 0;
    let mut temp = current.as_ref();
    while let Some(node) = temp {
        cnt += 1;
        temp = node.next.as_ref();
    }

    let mut prev_tail = &mut dummy;
    
    while cnt >= k {
        let mut group_nodes = Vec::new();
        
        // Collect k nodes
        for _ in 0..k {
            if let Some(mut node) = current.take() {
                current = node.next.take();
                group_nodes.push(node);
            }
        }
        
        // Reverse the group by connecting in reverse order
        for mut node in group_nodes.into_iter().rev() {
            prev_tail.next = Some(node);
            prev_tail = prev_tail.next.as_mut().unwrap();
        }
        
        cnt -= k;
    }
    
    // Connect remaining nodes
    if current.is_some() {
        prev_tail.next = current;
    }
    
    dummy.next
}