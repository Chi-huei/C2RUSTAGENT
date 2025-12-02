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
    
    // Check if we have at least k nodes
    let mut temp = head.as_ref();
    for _ in 0..k {
        if temp.is_none() {
            return head; // Not enough nodes, return original head
        }
        temp = temp.unwrap().next.as_ref();
    }
    
    // Clone the remaining part for recursion
    let remaining = if let Some(node) = temp {
        Some(node.clone())
    } else {
        None
    };
    
    // We have at least k nodes, proceed with reversal
    let mut current = head;
    let mut prev = reverse_k_group(remaining, k);
    
    // Reverse k nodes
    for _ in 0..k {
        if let Some(mut node) = current {
            current = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
    }
    
    prev
}