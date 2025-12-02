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

fn reverse_k(mut head: Box<ListNode>, k: i32) -> Box<ListNode> {
    if k <= 1 {
        return head;
    }
    
    let mut current = head.next.take();
    let mut prev = head;
    
    for _ in 1..k {
        if let Some(mut node) = current {
            current = node.next.take();
            node.next = Some(prev);
            prev = node;
        }
    }
    
    prev
}

fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || k <= 1 {
        return head;
    }
    
    // Check if we have at least k nodes
    let mut temp = &head;
    for _ in 0..k {
        match temp {
            Some(node) => temp = &node.next,
            None => return head, // Not enough nodes, return as is
        }
    }
    
    let mut head = head.unwrap();
    let remaining = head.next.take();
    
    // Store the original first node (will become last after reversal)
    let mut original_first = Box::new(ListNode::new(head.val));
    head.next = remaining;
    
    // Reverse k nodes
    let new_head = reverse_k(head, k);
    
    // Find the last node in the reversed group
    let mut last = &mut original_first;
    while last.next.is_some() {
        if let Some(ref mut next_node) = last.next {
            last = next_node;
            break;
        }
    }
    
    // Recursively process remaining nodes
    last.next = reverse_k_group(last.next.take(), k);
    
    Some(new_head)
}