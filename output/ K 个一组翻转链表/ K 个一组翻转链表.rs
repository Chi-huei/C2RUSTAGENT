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
    
    let head = head?;
    
    // Get length of the list
    let mut len = 1;
    let mut current = &head;
    while let Some(ref next_node) = current.next {
        len += 1;
        current = next_node;
    }
    
    // Number of groups to reverse
    let loop_count = len / k;
    if loop_count == 0 {
        return Some(head);
    }
    
    // Convert to vector for easier manipulation
    let mut nodes = Vec::new();
    let mut current = Some(head);
    while let Some(mut node) = current {
        current = node.next.take();
        nodes.push(node);
    }
    
    // Reverse groups
    for group in 0..loop_count {
        let start_idx = (group * k) as usize;
        let end_idx = start_idx + k as usize;
        nodes[start_idx..end_idx].reverse();
    }
    
    // Rebuild linked list
    for i in (0..nodes.len() - 1).rev() {
        nodes[i].next = Some(nodes.remove(i + 1));
    }
    
    Some(nodes.into_iter().next().unwrap())
}