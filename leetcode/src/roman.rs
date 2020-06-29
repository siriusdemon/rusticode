pub fn roman_to_int(s: String) -> i32 {
    let mut ans = 0;
    let mut prev: char = ' '; 
    for c in s.chars() {
        match c {
            'I' => ans = ans + 1,
            'V' => {
                ans = ans + 5;
                if prev == 'I' { ans = ans - 2 * 1; }
            },
            'X' => {
                ans = ans + 10;
                if prev == 'I' { ans = ans - 2 * 1; }
            }
            'L' => {
                ans = ans + 50;
                if prev == 'X' { ans = ans - 2 * 10; }
            },
            'C' => {
                ans = ans + 100;
                if prev == 'X' { ans = ans - 2 * 10; }
            },
            'D' => {
                ans = ans + 500;
                if prev == 'C' { ans = ans - 2 * 100; }
            },
            'M' => {
                ans = ans + 1000;
                if prev == 'C' { ans = ans - 2 * 100; }
            },
            _ => {
                return ans; // invalid
            }
        }
        prev = c;
    }
    ans
}

// https://leetcode-cn.com/problems/integer-to-roman/
pub fn int_to_roman(mut num: i32) -> String {
    let ints = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let strs = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

    let mut res = String::new();
    for i in 0..13 {
        while num >= ints[i] {
            res.push_str(strs[i]);
            num -= ints[i];
        }
    }
    return res;
}

fn main()
{
    let x = int_to_roman(3);
    println!("{}", x);
}