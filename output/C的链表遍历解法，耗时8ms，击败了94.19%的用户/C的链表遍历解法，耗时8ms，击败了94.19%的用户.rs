#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn reverse(head: Option<Box<ListNode>>, tail_val: i32) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut current = head;

    while let Some(mut node) = current {
        let next = node.next.take();
        node.next = prev;
        let is_tail = node.val == tail_val;
        prev = Some(node);
        if is_tail {
            break;
        }
        current = next;
    }

    prev
}

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || k <= 1 {
        return head;
    }

    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut prev_group_end = &mut dummy as *mut Box<ListNode>;

    loop {
        let mut current = unsafe { (*prev_group_end).next.as_ref() };
        let mut count = 0;

        // Check if we have k nodes
        while let Some(node) = current {
            count += 1;
            if count == k {
                break;
            }
            current = node.next.as_ref();
        }

        if count < k {
            break;
        }

        // Extract k nodes
        let group_start = unsafe { (*prev_group_end).next.take() };
        let mut nodes = Vec::new();
        let mut temp = group_start;

        for _ in 0..k {
            if let Some(mut node) = temp {
                temp = node.next.take();
                nodes.push(node);
            }
        }

        // Reverse the group
        let mut reversed_head: Option<Box<ListNode>> = None;
        for mut node in nodes.into_iter().rev() {
            node.next = reversed_head;
            reversed_head = Some(node);
        }

        // Connect the reversed group
        unsafe {
            (*prev_group_end).next = reversed_head;
        }

        // Find the end of the reversed group
        let mut group_end = unsafe { &mut (*prev_group_end).next };
        while group_end.as_ref().unwrap().next.is_some() {
            group_end = &mut group_end.as_mut().unwrap().next;
        }

        // Connect remaining nodes
        group_end.as_mut().unwrap().next = temp;

        // Update prev_group_end for next iteration
        prev_group_end = group_end.as_mut().unwrap() as *mut Box<ListNode>;
    }

    dummy.next
}