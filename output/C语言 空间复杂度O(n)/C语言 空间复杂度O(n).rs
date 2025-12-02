use std::ptr;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

const MY_OK: i32 = 0;
const MY_FAIL: i32 = -1;

struct MyStatus {
    lcnt: usize,
    larr: Vec<*mut ListNode>,
    k: usize,
    lsect_cnt: usize,
}

impl MyStatus {
    fn s_free(&mut self) {
        self.larr.clear();
    }

    fn s_init(&mut self, head: *mut ListNode, k: usize) -> i32 {
        let mut cur = head;
        self.lcnt = 0;
        
        while !cur.is_null() {
            self.lcnt += 1;
            unsafe {
                cur = (*cur).next.as_mut().map_or(ptr::null_mut(), |node| node.as_mut());
            }
        }
        
        if self.lcnt == 0 {
            return MY_OK;
        }
        
        self.larr = Vec::with_capacity(self.lcnt + 1);
        
        cur = head;
        for _ in 0..self.lcnt {
            self.larr.push(cur);
            unsafe {
                cur = (*cur).next.as_mut().map_or(ptr::null_mut(), |node| node.as_mut());
            }
        }
        self.larr.push(ptr::null_mut());
        
        self.lsect_cnt = self.lcnt / k;
        self.k = k;
        MY_OK
    }

    fn proc(&mut self) {
        for i in 0..self.lsect_cnt {
            let mut l = i * self.k;
            let mut r = (i + 1) * self.k - 1;
            
            while l < r {
                self.larr.swap(l, r);
                l += 1;
                r -= 1;
            }
        }
        
        for i in 0..self.lcnt {
            unsafe {
                if i + 1 < self.larr.len() && !self.larr[i + 1].is_null() {
                    (*self.larr[i]).next = Some(Box::from_raw(self.larr[i + 1]));
                } else {
                    (*self.larr[i]).next = None;
                }
            }
        }
    }
}

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || k <= 0 {
        return head;
    }
    
    let head_ptr = Box::into_raw(head.unwrap());
    let mut s = MyStatus {
        lcnt: 0,
        larr: Vec::new(),
        k: k as usize,
        lsect_cnt: 0,
    };
    
    let ret = s.s_init(head_ptr, k as usize);
    if ret != MY_OK {
        unsafe {
            Box::from_raw(head_ptr);
        }
        return None;
    }
    
    s.proc();
    let result = if !s.larr.is_empty() && !s.larr[0].is_null() {
        unsafe { Some(Box::from_raw(s.larr[0])) }
    } else {
        None
    };
    
    s.s_free();
    result
}