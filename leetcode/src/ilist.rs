// https://leetcode-cn.com/problems/merge-two-sorted-lists/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

// mory version
pub fn mory_merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if l1 == None {
        return l2;
    } else if l2 == None {
        return l1;
    } else {
        let mut l1 = l1.unwrap();
        let mut l2 = l2.unwrap();
        if l1.val <= l2.val {
            if let Some(rest) = merge_two_lists(l1.next.clone(), Some(l2)) {
                l1.next = Some(rest);
            }
            return Some(l1);
        } else {
            if let Some(rest) = merge_two_lists(Some(l1), l2.next.clone()) {
                l2.next = Some(rest);
            }
            return Some(l2);
        }
    }
}


// leetcoder
pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, r) => r,
        (l, None) => l,
        (Some(mut l), Some(mut r)) => {
            if l.val <= r.val {
                l.next = merge_two_lists(l.next, Some(r));   
                return Some(l);
            } else {
                r.next = merge_two_lists(Some(l), r.next);
                return Some(r);
            }
        }
    }
}

// https://leetcode-cn.com/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/
pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
    fn helper(head: Option<Box<ListNode>>, col: &mut Vec<i32>) {
        if let Some(node) = head {
            helper(node.next, col);
            col.push(node.val);
        }
    }
    let mut col = Vec::new(); 
    helper(head, &mut col);
    return col;
}

// https://leetcode-cn.com/problems/shan-chu-lian-biao-de-jie-dian-lcof/
// 我发现，当你 move 进一个列表的时候，你再也无法 move 出来了
pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    if head.is_none() { return head; }
    let mut head = head;

    let mut v = Vec::new();
    while let Some(node) = head {
        if node.val != val {
            let nn = Box::new(ListNode::new(node.val));
            v.push(nn);
        }
        head = node.next;
    }

    let mut last = v.pop();
    if last.is_none() { return None; }
    let mut last: Box<ListNode> = last.unwrap();

    for i in (0..v.len()).rev() {
        let mut node = v.pop().unwrap();
        node.next = Some(last);
        last = node;
    }
    return Some(last);
}

// https://leetcode-cn.com/problems/fan-zhuan-lian-biao-lcof/
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res = None;
    let mut p = head;
    while let Some(node) = p {
        let mut nn = ListNode::new(node.val);
        nn.next = res;
        res = Some(Box::new(nn));
        p = node.next;  
    }
    return res;
}

// https://leetcode-cn.com/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof/
// 非常难呀
pub fn get_kth_from_end(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut p = &head;
    let mut step = 0;
    for i in 0..k {
        if let Some(node) = p {
            p = &node.next;
        }
    }

    while let Some(node) = p {
        p = &node.next;
        step += 1;
    }

    for _ in 0..step {
        let node = head.unwrap();
        head = node.next;    
    }
    return head;
}

// https://leetcode-cn.com/problems/remove-duplicate-node-lcci/
pub fn remove_duplicate_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head
}

fn main()
{
    let mut l1 = ListNode::new(10);
    let mut l2 = ListNode::new(20);
    let l3 = ListNode::new(30);
    l2.next = Some(Box::new(l3)); 
    l1.next = Some(Box::new(l2));
    let list = Some(Box::new(l1));
    // // println!("{}", p.val);
    // // if let Some(q) = &p.next {
    // //     println!("{}", q.val);
    // }
    // dbg!(reverse_print(list));
    // delete_node(list, 4);
    // reverse_list(list);
    get_kth_from_end(list, 3);
}
