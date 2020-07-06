use std::collections::BinaryHeap;
use std::cmp::Reverse;
// https://leetcode-cn.com/problems/shu-ju-liu-zhong-de-zhong-wei-shu-lcof/solution/
struct MedianFinder {
    minheap: BinaryHeap<Reverse<i32>>,
    maxheap: BinaryHeap<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
        Self { minheap: BinaryHeap::new(), maxheap: BinaryHeap::new() }
    }
    
    fn add_num(&mut self, num: i32) {
        self.maxheap.push(num);
        let max = self.maxheap.pop().unwrap();
        self.minheap.push(Reverse(max));
        if self.minheap.len() > self.maxheap.len() + 1 {
            let Reverse(min) = self.minheap.pop().unwrap();
            self.maxheap.push(min);
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.minheap.len() == 0 {
            return -1.0;
        } else if self.minheap.len() > self.maxheap.len() {
            let &Reverse(m) = self.minheap.peek().unwrap(); 
            return m as f64;
        } else {
            let &Reverse(a) = self.minheap.peek().unwrap();
            let &b = self.maxheap.peek().unwrap();
            return (a as f64 + b as f64) / 2.;
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */


fn main()
{
    let num = 10;
    let mut obj = MedianFinder::new();
    obj.add_num(num);
    obj.add_num(11);
    obj.add_num(4);
    obj.add_num(5);
    let ret_2: f64 = obj.find_median();
    println!("{}", ret_2);
}