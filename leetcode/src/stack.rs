use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    if s.len() == 0 { return true; }
    let mut map = HashMap::new();
    map.insert(')', '(');
    map.insert(']', '[');
    map.insert('}', '{');
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match map.get(&c) {
            Some(&p) => match stack.pop() {
                Some(x) => { if x != p { return false; } },
                _ => return false,
            },
            _ => stack.push(c),
        }
    }
    return stack.len() == 0;
}


// https://leetcode-cn.com/problems/decode-string/
pub fn decode_string(s: String) -> String {
    let mut numstack: Vec<u32> = Vec::new();   
    let mut charstack: Vec<char> = Vec::new();   
    let mut res = String::new();
    let mut n = 0;

    for c in s.chars() {
        if c.is_ascii_alphabetic() && numstack.is_empty() {
            charstack.push(c);
        } else if c.is_digit(10) {
            let cn = c.to_digit(10).unwrap();
            n = if n == 0 { cn } else { n * 10 + cn };
        } else if c == '[' {
            numstack.push(n);
            n = 0;
            charstack.push(c);
        } else if c.is_ascii_alphabetic() {
            charstack.push(c);
        } else if c == ']' {
            // 一个 pat 已经入栈完整，现在将之出栈，注意，是逆序的
            let mut temp: Vec<char> = Vec::new();
            let mut temp_c = charstack.pop().unwrap();
            while temp_c != '[' {
                temp.push(temp_c);
                temp_c = charstack.pop().unwrap();
            }
            // println!("c={}, temp={:?}", c, temp);
            // 现在，解开了一个 pat，支持下一个 pat 的解析
            let times = numstack.pop().unwrap();
            // println!("times={}", times);
            for _ in 0..times {
                for i in (0..temp.len()).rev() {
                    charstack.push(temp[i]);
                }
                // println!("{:?}", charstack);
            }
        } else {
            println!("Some case escape! {}", c);
        }
    }
    // println!("{:?}", charstack);
    // NOTE: bad code
    for c in charstack {
        res.push(c);
    }
    println!("{}", res);
    return res;
}

fn main() {
    decode_string("3[a]2[bc]".to_string());
}