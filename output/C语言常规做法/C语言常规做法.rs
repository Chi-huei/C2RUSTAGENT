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
    let mut tmp = &head;
    let mut counter = k;
    while counter > 0 {
        match tmp {
            Some(node) => {
                tmp = &node.next;
                counter -= 1;
            }
            None => return head,
        }
    }
    
    // Reverse the first k nodes
    let mut current = head;
    let mut prev = None;
    counter = k;
    
    while counter > 0 && current.is_some() {
        let mut node = current.unwrap();
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        current = next;
        counter -= 1;
    }
    
    // Recursively reverse the rest and connect
    if let Some(mut first_node) = prev {
        // Find the tail of the reversed group (original head)
        let mut tail = &mut first_node;
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = reverse_k_group(current, k);
        Some(first_node)
    } else {
        prev
    }
}