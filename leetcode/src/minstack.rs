// the ministack https://leetcode-cn.com/problems/min-stack/solution/
pub struct MinStack {
    data: Vec<i32>,
    // helper stack
    mini: Vec<i32>,
    size: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    pub fn new() -> Self {
        MinStack {
            data: Vec::new(),
            mini: Vec::new(),
            size: 0,
        } 
    }
    
    pub fn push(&mut self, x: i32) {
        self.data.push(x);
        // check for the mini value
        // if stack is empty, just push into x
        if self.size == 0 {
            self.mini.push(x);
        // if min(x, mini.top) == mini.top
        } else if self.get_min() < x {
            self.mini.push(self.get_min());
        } else {
            self.mini.push(x);
        }

        self.size += 1;
    }
    
    pub fn pop(&mut self) {
        self.data.pop(); 
        self.mini.pop();
        self.size -= 1;
    }
    
    pub fn top(&self) -> i32 {
        return self.data[self.size-1];
    }
    
    pub fn get_min(&self) -> i32 {
        return self.mini[self.size-1];
    }
}

// fn main()
// {
//     let mut stack = MinStack::new();
//     stack.push(-2);
//     stack.push(0);
//     stack.push(03);
//     println!("min: {}", stack.get_min());
//     stack.pop();
//     stack.top();
//     println!("min: {}", stack.get_min());
// }