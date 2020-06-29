// kmp in Rust
pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack: Vec<char> = haystack.chars().collect();
    let next = compute_next(&needle);
    let needle: Vec<char> = needle.chars().collect();
    let mut j: usize = 0;
    let mut i: usize = 0; // index for haystack
    let mut index: usize = 0; // for return
    let len = haystack.len();
    let len_n = next.len();


    // 这里有三种情况需要判断
    while i < len && j < len_n {
        if haystack[i] == needle[j] {
            if j == 0 { index = i; } // fresh match update index
            i += 1;
            j += 1;
        } else if j > 0 {       // if j == 0, no next[j] to dump
            j = next[j] - 1;    // jump to next[j], continue to match
            index = i - j;
        } else {                // here, nor match either j > 0
            i += 1;             // so, match next i
        }
    }
    // if found
    if j == len_n { return index as i32; }
    // no found
    -1
}


pub fn compute_next(needle: &str) -> Vec<usize> {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let len = needle.len();

    // 判空
    if len == 0 { return vec![]; }

    let P: Vec<char> = needle.chars().collect();
    let mut next = vec![0];  


    // 如果上面已经判断非空，这里用 i < len - 1 也不怕
    while i+1 < len {
        if j == 0 || P[i] == P[j-1] {
            i += 1;
            j += 1;
            next.push(j);
        } else {
            j = next[j-1];
        }
    }
    println!("Finish compute next: \n");
    println!("{:?}", next);

    next
} 

