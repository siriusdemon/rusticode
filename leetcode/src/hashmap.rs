use std::collections::HashMap;

// https://leetcode-cn.com/problems/intersection-of-two-arrays-ii/solution/
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for n1 in &nums1 {
        if map.contains_key(n1) {
            let v = map.get_mut(n1).unwrap();
            *v += 1;
        } else {
            map.insert(*n1, 1);
        }
    }
    
    let mut res: Vec<i32> = Vec::new();
    for n2 in nums2 {
        if let Some(v) = map.get_mut(&n2) {
            if *v > 0 {
                res.push(n2);
                *v -= 1;
            }
        }
    }
    return res;
}

fn main()
{
    let nums1 = [1,2,2,1].to_vec();
    let nums2 = [2,2].to_vec();
    let res = intersect(nums1, nums2);
    println!("{:?}", res);
}