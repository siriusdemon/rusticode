// https://leetcode-cn.com/problems/lru-cache/solution/
use std::collections::{HashMap, VecDeque};

struct LRUCache {
    map: HashMap<i32, usize>,
    qind: VecDeque<usize>,
    qval: VecDeque<i32>,
    capacity: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            map: HashMap::with_capacity(capacity),
            qind: VecDeque::with_capacity(capacity), 
            qval: VecDeque::with_capacity(capacity), 
            capacity,
        }
    }
    
    fn get(&self, key: i32) -> i32 {
        let ind = self.map.get(&key);
        match ind {
            Some(&i) => self.qval[self.qind[i]],
            None => -1,
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
      
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

fn main()
{
    let capacity = 10;
    let mut obj = LRUCache::new(capacity);
    obj.put(100, 2); 
    obj.put(12, 2);
}