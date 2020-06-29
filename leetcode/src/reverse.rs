// reverse integer
// 基本思路： 
// 将整数按个位拆来，一边拆一边组成新的数。每次生成下一个新数之前，都要先判断一下现在
// 的新数乘10之后会不会溢出。
pub fn reverse_int(x: i32) -> i32 {
    let mut ans = 0;
    let mut x = x;
    println!("x {}", x);
    while x != 0 {
        let pop = x % 10;
        if ans > std::i32::MAX / 10 || (ans == std::i32::MAX / 10 && pop > 7) {
            return 0;
        } 
        if ans < std::i32::MIN / 10 || (ans == std::i32::MIN / 10 && pop < -8) {
            return 0;
        }
        ans = ans * 10 + pop;
        x = x / 10;
        println!("ans {} pop {} x {}", ans, pop, x);
    }
    return ans;
}

pub fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    let mut t: char = ' '; 

    for i in 0..len/2 {
        t = s[i];
        s[i] = s[len - i - 1];
        s[len - i - 1] = t;
    }
}

// https://leetcode-cn.com/problems/reverse-words-in-a-string-iii/
pub fn reverse_words(s: String) -> String {
    let vs: Vec<String> = s.split_whitespace()
                            .map(|x| x.chars()
                            .rev()
                            .collect::<String>())
                            .collect();
    return vs.join(" ");
}