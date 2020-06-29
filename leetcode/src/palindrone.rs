pub fn is_palindrome(x: i32) -> bool {
    let mut x_mut = x;
    let mut ans = 0;

    if x < 0 { return false; }

    while x_mut != 0 {
        ans = ans * 10 + x_mut % 10;
        x_mut = x_mut / 10;
    }
    return x == ans;
}

fn center_expand(chars: &Vec<char>, center: usize) -> usize {
    let len = chars.len();
    if center == 0 || center == len -1 {
        return 1;
    }
    let mut left = center - 1;
    let mut right = center + 1;
    while 0 < left && right < len -1 && chars[left] == chars[right] {
        left -= 1;
        right += 1;
    }
    if chars[left] == chars[right] {
        return right - left + 1;
    } else {
        return right - left - 1;
    }
}
// https://leetcode-cn.com/problems/longest-palindromic-substring/solution/5-zui-chang-hui-wen-zi-chuan-cc-by-bian-bian-xiong/
pub fn longest_palindrome(s: String) -> String {
    let len = s.len();
    if len <= 1 { return s; }

    let ss: Vec<_> = s.split("").collect();
    let ss = ss.join("#");
    let chars: Vec<_> = ss.chars().collect();
    let new_len = chars.len();

    for i in 0..new_len {
        print!("{:<3}", i);
    }
    println!();
    for i in 0..new_len {
        print!("{:3}", chars[i]);
    }
    println!();

    let mut start = 0;
    let mut true_len = 1;
    let mut maxlen = 1;

    
    for i in 1..new_len-1 {
        let length = center_expand(&chars, i);
        if length > maxlen {
            maxlen = length;
            true_len = length / 2;  // remove #
            start = (i - 1) / 2 - (true_len - 1) / 2;
        }
    }
    return s.chars().skip(start).take(true_len).collect();
}

pub fn longest_palindrome_better(s: String) -> String {
    let len = s.len();
    if len <= 1 {
        return s;
    }
    
    let chars: Vec<char> = s.chars().collect();
    let (mut index, mut max, mut start, mut end) = (0, 0, 0, 0);
    while index < len {
        let (mut left, mut right) = (index, index);
        let center = chars[index];
        let mut index_plus = 1;
       
        // 从中心向左右扩展，首先解决左右两个索引值相同的问题，这里顺便
        // 解决奇偶对称问题
        while left > 0 && chars[left - 1] == center {
            left -= 1;
        }
        // index_plus 解决同一个数字重复出现重复测试的问题。
        while right < len - 1 && chars[right + 1] == center {
            right += 1;
            index_plus += 1;
        }
        // 中心扩展
        while left > 0 && right < len - 1 && chars[left - 1] == chars[right + 1] {
            left -= 1;
            right += 1;
        }
        
        let length = right - left + 1;
        if length > max {
            max = length;
            start = left;
            end = right;
        }
        
        if length >= len - 1 {
            break;
        }
        // 更新中心
        index += index_plus;
    }
    return s[start..=end].to_owned();
}

fn main()
{
    let s = "aabaaaaaaaaaabax".to_string();
    let x = longest_palindrome_better(s);
    println!("{}", x);
}