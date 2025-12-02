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
    
    let k = k as usize;
    
    // Convert to vector for easier manipulation
    let mut values = Vec::new();
    let mut current = head;
    while let Some(node) = current {
        values.push(node.val);
        current = node.next;
    }
    
    if values.len() < k {
        return build_list(&values);
    }
    
    // Reverse in groups of k
    let mut i = 0;
    while i + k <= values.len() {
        values[i..i + k].reverse();
        i += k;
    }
    
    build_list(&values)
}

fn build_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in values.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }
    head
}