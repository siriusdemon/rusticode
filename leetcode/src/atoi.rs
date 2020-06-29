// 将字符串转成整数
pub fn atoi(string: String) -> i32 {
    println!("String: {}", string);
    let mut ans: i32 = 0;
    let mut start_parse: bool = false;
    let mut sign = 1; // 正负号
    for c in string.chars() {
        if c.is_whitespace() && start_parse == false { 
            continue; 
        } else if c == '-' && start_parse == false {
            start_parse = true; 
            sign = -1;
        } else if c == '+' && start_parse == false {
            start_parse = true;
        } else if let Some(i) = c.to_digit(10) {
            let i: i32 = i as i32;
            start_parse = true;
            if sign == 1 && ((ans > std::i32::MAX / 10) || (ans == std::i32::MAX / 10 && i > 7)) {
                return std::i32::MAX;
            } else if sign == -1 && ((ans * sign < std::i32::MIN / 10) || (sign * ans == std::i32::MIN / 10 && i > 8)) {
                return std::i32::MIN;
            } 
            ans = ans * 10 + i;
            println!("sign {} ans {} c {}", sign, ans, c);
        } else {
            break;
        }
    }
    ans * sign
}


// https://leetcode-cn.com/problems/biao-shi-shu-zhi-de-zi-fu-chuan-lcof/solution/que-ding-you-xian-zi-dong-ji-dfa-by-justyou/
// 合法的数字: 123 -> 123.121 -> +.1 -> 0.1 -> 4e5 -> 1.0e10 -> 1.23e-4 -> 1.e5
// 有限状态机
// ------ 字符类型
// 空格、数字 0—9, 正负号 -/+, 小数点, 幂符号 e  
// ------ 九种状态
// 0开始的空格 -> 0空格，1正负号，2数字，3小数点
// 1幂符号前的正负号 -> 2数学，3小数点 
// 2小数点前的数字 -> 2数字，3小数点
// 3小数点-> 4数字，5幂
// 4小数点后的数字 -> 4数字, 5幂
// 5幂符号 -> 6正负号， 7数字
// 6幂符号后的正负号 -> 7数字
// 7幂符号后的数字 -> 7数字，8空格
// 8结尾的空格 -> 8空格

// ------ 结束状态
// 2 3 4 7 8

pub fn is_number(s: String) -> bool {
    true
}
