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

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 {
        return head;
    }
    
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    
    let mut prev_group_end = &mut dummy;
    
    loop {
        let group_start = prev_group_end.next.take();
        if group_start.is_none() {
            break;
        }
        
        let mut nodes = Vec::new();
        let mut current = group_start;
        
        for _ in 0..k {
            match current {
                Some(mut node) => {
                    current = node.next.take();
                    nodes.push(node);
                }
                None => {
                    let mut restored = None;
                    for node in nodes.into_iter().rev() {
                        let mut node = node;
                        node.next = restored;
                        restored = Some(node);
                    }
                    prev_group_end.next = restored;
                    return dummy.next;
                }
            }
        }
        
        let remaining = current;
        
        for i in (0..nodes.len()).rev() {
            if i == 0 {
                nodes[i].next = remaining.clone();
            } else {
                nodes[i].next = Some(nodes[i - 1].clone());
            }
        }
        
        prev_group_end.next = Some(nodes[nodes.len() - 1].clone());
        
        for _ in 0..k {
            prev_group_end = prev_group_end.next.as_mut().unwrap();
        }
    }
    
    dummy.next
}