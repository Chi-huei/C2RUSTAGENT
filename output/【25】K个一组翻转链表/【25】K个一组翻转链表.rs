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
    
    // Check if we have k nodes
    let mut cur = &head;
    let mut count = 0;
    while cur.is_some() && count < k {
        cur = &cur.as_ref().unwrap().next;
        count += 1;
    }
    
    if count == k {
        // We have k nodes, so reverse them
        let mut head = head;
        let mut cur = reverse_k_group(take_next(&mut head, k), k);
        
        // Reverse the first k nodes
        for _ in 0..k {
            let mut node = head.take().unwrap();
            let next = node.next.take();
            node.next = cur;
            cur = Some(node);
            head = next;
        }
        cur
    } else {
        // Less than k nodes remaining, return as is
        head
    }
}

fn take_next(head: &mut Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut cur = head;
    for _ in 0..k {
        if let Some(node) = cur {
            cur = &mut node.next;
        } else {
            break;
        }
    }
    cur.take()
}