use std::vec::Vec;
use std::collections::HashMap;


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len()-1 {
        let res = target - nums[i];
        for j in i+1..nums.len() {
            if res == nums[j] {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}


pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash: HashMap<i32, i32> = HashMap::new();
    for (i, &v) in nums.iter().enumerate() {
        hash.insert(v, i as i32);
    }

    for (i_, &v) in nums.iter().enumerate() {
        if let Some(&i) = hash.get(&(target - v)){
            if i != i_ as i32 {
                return vec![i_ as i32, i as i32];
            }
        }
    }
    return Vec::new();
}


// https://leetcode-cn.com/problems/3sum/solution/
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 { return Vec::new(); }
    
    let mut res = Vec::new();
    let mut nums = nums;
    nums.sort();
    for i in 0..nums.len() {
        if nums[i] > 0 { 
            return res;
        } else if i > 0 && nums[i] == nums[i-1] {
            continue;
        } else {
            let mut L = i + 1;
            let mut R = nums.len() - 1;
            while L < R {
                let sum = nums[i] + nums[L] + nums[R];
                if sum == 0 {
                    let r = [nums[i], nums[L], nums[R]].to_vec();
                    res.push(r);
                    while L < R && nums[L] == nums[L+1] {
                        L += 1;
                    }
                    while L < R && nums[R] == nums[R-1] {
                        R -= 1;
                    }
                    L += 1;
                    R -= 1;
                } else if sum < 0 {
                    L += 1;
                } else {
                    R -= 1;
                }
            }
        }
    } 
    return res;
}