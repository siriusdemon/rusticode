pub struct MyQueue {
    data: Vec<i32>,
    top: i32,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            data: Vec::new(),
            top: -1,
        }        
    }

    pub fn push_back(&mut self, x: i32) {
        self.data.push(x); 
        if self.top == -1 { self.top = 0; }
    }

    pub fn pop(&mut self) -> i32 {
        let val = self.top(); 
        self.top += 1;
        return val;
    }

    pub fn top(&self) -> i32 {
        if !self.is_empty() {
            return self.data[self.top as usize];
        }    
        return -1111111111;
    }

    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }

    pub fn len(&self) -> usize {
        if self.top == -1 {
            return 0;
        }
        return self.data.len() - self.top as usize;
    }

    pub fn squeeze(mut self) -> Self {
        if self.is_empty() {
            return Self::new();
        }
        let mut new = Self::new();
        for i in self.top..(self.len() as i32 + self.top) {
            new.push_back(self.pop());
        }
        return new;
    }
}



// https://leetcode-cn.com/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/
// 用两个栈模拟队列，一个负责入，一个负责出。关键点在于，只有当出栈为空时，才能把入栈的所有元素
// 移入出栈
// lazy stack，只有当需要出的时候，才把入栈的内容移进出栈
struct CQueue {
    instack: Vec<i32>,
    outstack: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {

    fn new() -> Self {
        Self { instack: Vec::new(), outstack: Vec::new(), }
    }
    
    fn append_tail(&mut self, value: i32) {
        self.instack.push(value);
    }
    
    fn delete_head(&mut self) -> i32 {
        if self.outstack.len() > 0 {
            let value = self.outstack.pop().unwrap();
            return value;
        } else if self.instack.len() > 0 {
            while let Some(value) = self.instack.pop() {
                self.outstack.push(value);
            }
            return self.delete_head();
        } else {
            return -1;
        }
    }
}

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */


fn main()
{
    let mut q = MyQueue::new();
    println!("Init Queue {}", q.len());
    println!("Empty Queue? {}", q.is_empty());
    q.push_back(1314);
    println!("len {}, top {}", q.len(), q.top());
    q.push_back(520);
    q.pop();
    println!("len {}, top {}", q.len(), q.top());
    q.push_back(666);
    println!("len {}, top {}", q.len(), q.pop());
    // println!("len {}, top {}", q.len(), q.pop());
    println!("true data len {}", q.data.len());
    let mut p = q.squeeze();
    println!("true data len {}", p.data.len());
    // println!("true data len {}", q.data.len());
}

