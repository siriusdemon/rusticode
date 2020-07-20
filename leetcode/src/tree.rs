use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}



// https://leetcode-cn.com/problems/same-tree/
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn same_tree(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, r) => false,
            (l, None) => false,
            (Some(l), Some(r)) => {
                let l = l.borrow();
                let r = r.borrow();
                return l.val == r.val && same_tree(&l.left, &r.left) && same_tree(&l.right, &r.right);
            }
        }
    }
    return same_tree(&p, &q);
}



// https://leetcode-cn.com/problems/symmetric-tree/solution/
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_mirror(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(l), None) => false,
            (None, Some(r)) => false,
            (Some(l), Some(r)) => {
                let l = l.borrow();
                let r = r.borrow();
                return l.val == r.val && is_mirror(&l.left, &r.right) && is_mirror(&l.right, &r.left);
            }
        }
    }
    match root {
        None => true,
        Some(r) => {
            let r = r.borrow();
            return is_mirror(&r.left, &r.right);
        }
    }
}



// https://leetcode-cn.com/problems/kth-smallest-element-in-a-bst/solution/
pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn mid_traverse(root: Option<Rc<RefCell<TreeNode>>>, k: usize, cache: &mut Vec<i32>) {
        match root {
            None => (),
            Some(mut node) => {
                let mut node = node.borrow_mut();
                let right = node.right.take();
                let left = node.left.take();
                if k == cache.len() { return; }
                mid_traverse(left, k, cache);
                cache.push(node.val);
                mid_traverse(right, k, cache);
            }
        }
    }
    let k = k as usize;
    let mut cache = Vec::new();
    mid_traverse(root, k, &mut cache);
    return cache[k-1];
}

// https://leetcode-cn.com/problems/er-cha-sou-suo-shu-de-di-kda-jie-dian-lcof/
pub fn kth_largest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn mid_traverse(root: Option<Rc<RefCell<TreeNode>>>, k: usize, cache: &mut Vec<i32>) {
        match root {
            None => (),
            Some(mut node) => {
                let mut node = node.borrow_mut();
                let right = node.right.take();
                let left = node.left.take();
                if k == cache.len() { return; }
                mid_traverse(right, k, cache);
                cache.push(node.val);
                mid_traverse(left, k, cache);
            }
        }
    }
    let k = k as usize;
    let mut cache = Vec::new();
    mid_traverse(root, k, &mut cache);
    return cache[k-1];
}

// https://leetcode-cn.com/problems/check-balance-lcci/
pub fn is_balanced2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut flag = true;
    fn depth(root: &Option<Rc<RefCell<TreeNode>>>, flag: &mut bool) -> usize {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let ld = depth(&node.left, flag);
                let rd = depth(&node.right, flag);
                if ld > rd && (ld - rd) > 1 { *flag = false; }
                else if ld < rd && (rd - ld) > 1 { *flag = false; }
                return ld.max(rd) + 1
            }
        }
    }
    depth(&root, &mut flag);
    return flag;
}


// https://leetcode-cn.com/submissions/detail/79895095/
pub fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    match root {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            let ld = depth(&node.left);
            let rd = depth(&node.right);
            return ld.max(rd) + 1;
        }
    }
}


// https://leetcode-cn.com/problems/er-cha-shu-de-jing-xiang-lcof/
pub fn mirror_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    // how to return a tree;
    match root {
        None => root,
        Some(node) => { 
            // 放进这里，是因为Rust无法检测到我的node_ref实际上已经不会再使用了。
            // 加个 {} 引导它 drop 掉引用
            {
                let mut node_ref = node.borrow_mut();
                let left = node_ref.left.take();
                let right = node_ref.right.take();
                let mleft = mirror_tree(left);
                let mright = mirror_tree(right);
                node_ref.left = mright;
                node_ref.right = mleft;
            }
           return Some(node);
        }
    }
}

// https://leetcode-cn.com/problems/shu-de-zi-jie-gou-lcof/
pub fn is_sub_structure(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn helper(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => true,
            (Some(at), Some(bt)) => {
                let atr = at.borrow();
                let btr = bt.borrow();
                return (atr.val == btr.val) && helper(&atr.left, &btr.left) && helper(&atr.right, &btr.right);
            }
        }
    }

    fn helper2(a: Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if a.is_none() || b.is_none() {
            return false;
        } else {
            let c = helper(&a, b);
            if c { return c; } // else
            let a = a.unwrap();
            let mut a = a.borrow_mut();
            let left = a.left.take();
            let right = a.right.take();
            return helper2(left, b) || helper2(right, b);
        }
    }
    return helper2(a, &b);
}

// https://leetcode-cn.com/problems/zhong-jian-er-cha-shu-lcof/
use std::collections::HashMap;
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() { return None; }
    let mut table = HashMap::new();
    for (i, &v) in inorder.iter().enumerate() {
        table.insert(v, i);
    }
    // 前序中的根，在中序中的树的范围，前序中的根不一定跟中序中的左边界一样，这里右边界不包含
    fn helper(r: usize, p: usize, q: usize, preorder: &Vec<i32>, inorder: &Vec<i32>, table: &HashMap<i32, usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if p == q { return None; }
        let root = preorder[r];
        let &i = table.get(&root).unwrap();
        // 听着，你在前序的根是 r+1 而你中序的范围是 p, i-1
        let left = helper(r+1, p, i, preorder, inorder, table);
        let left_tree_size = i - p;
        let right = helper(r + left_tree_size + 1, i+1, q, preorder, inorder, table);
        let mut tree = TreeNode::new(root);
        tree.left = left;
        tree.right = right;
        return Some(Rc::new(RefCell::new(tree)));
    }
    let q = preorder.len();
    // 只有第一次的时候，根与左边界是一样的
    return helper(0, 0, q, &preorder, &inorder, &table);
}

// https://leetcode-cn.com/problems/cong-shang-dao-xia-da-yin-er-cha-shu-lcof/submissions/
// 树的中序遍历，要用到队列
use std::collections::VecDeque;
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    if root == None { return res; }

    let mut queue = VecDeque::new();
    queue.push_back(root);
    while let Some(node) = queue.pop_front() {
        if let Some(node) = node {
            let mut node = node.borrow_mut();
            res.push(node.val);
            queue.push_back(node.left.take());
            queue.push_back(node.right.take());
        }
    }
    return res;
}
// https://leetcode-cn.com/problems/cong-shang-dao-xia-da-yin-er-cha-shu-ii-lcof/
// 返回的方式不同
pub fn level_order2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root == None { return Vec::new(); }
    let mut res = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back(root);
    while queue.len() > 0 {
        let mut level = Vec::new();
        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap();
            if let Some(node) = node {
                let mut node = node.borrow_mut();
                level.push(node.val);
                queue.push_back(node.left.take());
                queue.push_back(node.right.take());
            }
        }
        if level.len() > 0 {
            res.push(level);
        }
   }
    return res;
}

// https://leetcode-cn.com/problems/cong-shang-dao-xia-da-yin-er-cha-shu-iii-lcof/
// 之字形打印
pub fn level_order3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root == None { return Vec::new(); }
    // true 时，要从左往右打印，则下一行要从右往左打印，那么，我应该把新来的数据放在前头
    let mut flag = true; 
    let mut res = Vec::new();
    return res;
}

// https://leetcode-cn.com/problems/er-cha-sou-suo-shu-de-hou-xu-bian-li-xu-lie-lcof/
// 后序遍历，需要切割树
pub fn verify_postorder(postorder: Vec<i32>) -> bool {
    let len = postorder.len();
    if len <= 1 { return true; }
    fn helper(postorder: &Vec<i32>, p: usize, q:usize) -> bool {
        if q == p { return true; }
        let root = postorder[q-1];
        println!("root = {}", root);
        let mut i = p;
        while i < q && postorder[i] < root {
            i += 1; 
        }
        for j in i..q {
            if postorder[j] < root {
                return false;
            }
        }
        // 只有单子树了
        if i == q || i == p {
            return helper(postorder, p, q-1); 
        } else {
            // 有左右子树，左为p-i, 右为i q-1
            // 验证一下是否 root 的左右节点满足条件
            // 左树的根为 i-1，这个已经由 while 循环验证了
            // 右树的根为 q-2;
            return  helper(postorder, p, i) && helper(postorder, i, q-1);
        }
    }
    return helper(&postorder, 0, len);
}

// https://leetcode-cn.com/problems/er-cha-shu-zhong-he-wei-mou-yi-zhi-de-lu-jing-lcof/
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, mut sv: Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if sum == node.val && node.left.is_none() && node.right.is_none() {
                sv.push(node.val);
                res.push(sv);
            } else {
                sv.push(node.val);
                let left = node.left.take();
                let right = node.right.take();
                if left.is_none() {
                    helper(right, sum - node.val, sv, res);
                } else if right.is_none() {
                    helper(left, sum - node.val, sv, res);
                } else {
                    helper(left, sum - node.val, sv.clone(), res);
                    helper(right, sum - node.val, sv, res);
                }
            }
        }
    }
    let mut res = Vec::new();
    let mut sv = Vec::new();
    helper(root, sum, sv, &mut res);
    return res;
}

// https://leetcode-cn.com/problems/path-sum/
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
    match root {
        Some(node) => {
            let mut node = node.borrow_mut();
            if node.left.is_none() && node.right.is_none() {
                return sum == node.val;
            } else {
                let res = sum - node.val; 
                let left = node.left.take();
                let right = node.right.take();
                return has_path_sum(left, res) | has_path_sum(right, res); 
            }
        },
        None => false
    }
}


// https://leetcode-cn.com/problems/minimum-depth-of-binary-tree/
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let mut node = node.borrow_mut();
            let ld = min_depth(node.left.take());
            let rd = min_depth(node.right.take());
            if ld > 0 && rd > 0 {
                return 1 + ld.min(rd);
            } else {
                return 1 + ld + rd;
            }
        }
    }
}

// https://leetcode-cn.com/problems/convert-sorted-array-to-binary-search-tree/
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(nums: &Vec<i32>, p: usize, q:usize) -> Option<Rc<RefCell<TreeNode>>> {
        if p == q { return None; }
        let m = (q + p) / 2;
        let mut root = TreeNode::new(nums[m]); 
        let left = helper(nums, p, m);
        let right = helper(nums, m+1, q);
        root.left = left;
        root.right = right;
        return Some(Rc::new(RefCell::new(root)));
    }    
    return helper(&nums, 0, nums.len());
}

// https://leetcode-cn.com/problems/invert-binary-tree/
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => root,
        Some(mut node) => {
            {
                let mut node = node.borrow_mut();
                let left = invert_tree(node.left.take());
                let right = invert_tree(node.right.take());
                node.right = left;
                node.left = right;
            } 
           return Some(node);
        }
    }
}

// https://leetcode-cn.com/problems/er-cha-shu-ren-wu-diao-du/
// 任务树，先串行，后并行。是最优的。
// 现有左右子树，左树的任务时间和为 a， 最大并行时间为 b 
// 右树的为 c，d
// 左右的任务时间为 a + c，根任务的时间为 val
// 根树的最大并行时间，应该是左右子树都能并行时取到，则为 (a + c) / 2，但不一定取到。取到的条件是一直并行！
// 但不失一般化，设 a >= c，左树总是比右树久一些。那么应该让左树先走一段，剩下的时间与 c 等，此时再左右并行
// 左树走多久？因为左树的最大并行时间为 b，先并行一下看看， 于是剩 a - 2b
// 现在有两个要讨论的，a - 2b == c，则完美并行。此时并行时间是 (a + c) / 2
// a - 2b < c，这可以通过缩小左树并行计算的时候，剩余 c 的工作量，于是回到上式，并行时间是 （a + c) / 2
// a - 2b > c，如此，则右树仍先完成，左树完成他的 (a - 2b - c)，此时最大并行时间是 b + c，即充分利用左树的并行时间，并将右树纳入并行。
pub fn minimal_exec_time(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> (f64, f64) {
        match root {
            None => (0.0, 0.0),
            Some(node) => {
                let node = node.borrow();
                let (mut a, mut b) = helper(&node.left);
                let (mut c, mut d) = helper(&node.right);
                // 交换使左树总比右树大
                if a < c {
                    let mut t = a;
                    a = c;
                    c = t;
                    t = b;
                    b = d;
                    d = t;
                }
                let total = node.val as f64 + a + c;
                let paral = if (a - 2.0 * b) <= c { (a + c) / 2.0 } else { b + c };
                return (total, paral);
            }
        }
    }
    let (total, paral) = helper(&root);
    return total - paral; 
}

// https://leetcode-cn.com/problems/binary-tree-maximum-path-sum/
// 最大树径和
use std::i32;
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn max_sum(root: &Option<Rc<RefCell<TreeNode>>>, maxsum: &mut i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let left = max_sum(&node.left, maxsum).max(0);
                let right = max_sum(&node.right, maxsum).max(0);
                let sum = node.val + left + right;
                *maxsum = sum.max(*maxsum);
                return node.val + left.max(right);
            }
        }
    }    
    let mut maxsum = i32::MIN;
    max_sum(&root, &mut maxsum);
    return maxsum;
}


fn main()
{
    let tree = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    
    // println!("{:?}", tree);
    // println!("{:?}", left);
    
    let tree = tree.unwrap();
    tree.borrow_mut().left = left;
    tree.borrow_mut().right = right;
    let tree = Some(tree);

    // println!("{:?}", tree);
    // println!("{:?}", tree.borrow_mut());
    // println!("{:?}", tree.borrow_mut().left);
    // println!("{:?}", tree.borrow_mut().right);
    // println!("balances {}", is_balanced(Some(tree)));
    // println!("same tree? {}", is_same_tree(Some(tree.clone()), Some(tree)));
    // println!("symmetric tree? {}", is_symmetric_iter(Some(tree)));
    // dbg!(depth(&tree));
    // dbg!(is_balanced2(tree));
    // let ytree = tree.clone();
    // is_sub_structure(tree, ytree);
    // build_tree([3,9,20,15,7].to_vec(), [9,3,15,20,7].to_vec());
    // dbg!(build_tree([1,2,3].to_vec(), [3,2,1].to_vec()));
    // build_tree([1,2,3].to_vec(), [2,3, 1].to_vec());
    // [4, 6, 7, 5]
    // verify_postorder([4,6,7,5].to_vec());
    // dbg!(invert_tree(tree));
    dbg!(minimal_exec_time(tree));
}