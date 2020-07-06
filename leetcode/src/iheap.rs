use std::collections::BinaryHeap;
use std::cmp::Reverse;
// https://leetcode-cn.com/problems/shu-ju-liu-zhong-de-zhong-wei-shu-lcof/solution/
struct MedianFinder {
    minheap: BinaryHeap<i32>,
    maxheap: BinaryHeap<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
        minheap: BinaryHeap::new(),
        maxheap: BinaryHeap::new(),
    }
    
    fn add_num(&mut self, num: i32) {
        self.maxheap.push(num);
        if self.maxheap.len() > self.minheap.len() {
            let max = self.maxheap.pop().unwrap();
            self.minheap.push(max);
        }
    }
    
    fn find_median(&self) -> f64 {

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
}