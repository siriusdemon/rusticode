// https://leetcode-cn.com/problems/count-and-say/solution/
pub fn say(s: String) -> String {
    let mut res = String::new();
    let len = s.len();
    let chars: Vec<char> = s.chars().collect();
    let mut count = 1;
    for i in 0..len-1 {
        if chars[i] != chars[i+1] {
            let tmp = format!("{}{}", count, chars[i]);
            res = res + &tmp;
            count = 1; // reset
        } else {
            count += 1;
        }
    }
    let tmp = format!("{}{}", count, chars[len-1]);
    res = res + &tmp;
    return res;
}

pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        return "1".to_string();
    } else {
        return say(count_and_say(n-1));
    }
}

// fn main()
// {
//     // let res = say("1121".to_string());
//     // println!("{}, and true is 211211", res);
//     // let res = say("112111".to_string());
//     // println!("{}, and true is 211231", res);
//     for i in 1..30 {
//         println!("{}", count_and_say(i));
//     }
// }