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

fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut num = 0;
    let mut node = &head;
    while let Some(ref current) = node {
        num += 1;
        node = &current.next;
    }
    
    if num < k {
        return head;
    }
    
    let mut current = head;
    let mut prev: Option<Box<ListNode>> = None;
    let mut n = k;
    
    while n > 0 && current.is_some() {
        let mut node = current.unwrap();
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        current = next;
        n -= 1;
    }
    
    if let Some(ref mut tail) = prev {
        let mut tail_ref = tail;
        while tail_ref.next.is_some() {
            tail_ref = tail_ref.next.as_mut().unwrap();
        }
        tail_ref.next = reverse_k_group(current, k);
    }
    
    prev
}