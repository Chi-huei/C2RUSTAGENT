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
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }
    
    reverse(head, k as usize)
}

fn reverse(head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
    let mut temp: Vec<Option<Box<ListNode>>> = Vec::with_capacity(k);
    let mut current = head;
    let mut i = 0;
    
    // Collect k nodes
    for _ in 0..k {
        if let Some(mut node) = current {
            current = node.next.take();
            temp.push(Some(node));
            i += 1;
        } else {
            break;
        }
    }
    
    if i < k {
        // Not enough nodes, return the first node with original connections restored
        if i == 0 {
            return None;
        } else {
            // Restore connections
            for j in 0..i-1 {
                let next_node = temp[j + 1].take();
                if let Some(ref mut node) = temp[j] {
                    node.next = next_node;
                }
            }
            return temp[0].take();
        }
    } else {
        // Reverse the k nodes
        let mut result = temp[k-1].take();
        
        for j in (1..k).rev() {
            let mut prev_node = temp[j-1].take();
            if let Some(ref mut prev) = prev_node {
                prev.next = result;
            }
            result = prev_node;
        }
        
        // Connect to the recursively processed rest
        if let Some(ref mut node) = temp[0] {
            node.next = reverse(current, k);
        }
        
        result
    }
}