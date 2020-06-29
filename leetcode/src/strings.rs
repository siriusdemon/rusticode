// https://leetcode-cn.com/problems/add-strings/solution/add-strings-shuang-zhi-zhen-fa-by-jyd/
pub fn add_strings(num1: String, num2: String) -> String {
    let num1: Vec<char> = num1.chars().collect();
    let num2: Vec<char>  = num2.chars().collect();

    let l1: usize = num1.len();
    let l2: usize = num2.len();
    let count = l1.max(l2);

    let mut carry: u32 = 0;
    let mut result = String::new();
    for c in 1..=count {
        let n1 = if l1 >= c { num1[l1 - c].to_digit(10).unwrap() } else { 0 };
        let n2 = if l2 >= c { num2[l2 - c].to_digit(10).unwrap() } else { 0 };
        let tmp = n1 + n2 + carry;
        carry = tmp / 10;
        result = format!("{}{}", (tmp%10), result);
    }
    if carry > 0 {
        result = format!("1{}", result);
    }
    return result;
}

// https://leetcode-cn.com/problems/multiply-strings/solution/
// pub fn multiply(num1: String, num2: String) -> String {

// }

// https://leetcode-cn.com/problems/add-binary/
pub fn add_binary(a: String, b: String) -> String {
    let num1: Vec<char> = a.chars().collect();
    let num2: Vec<char>  = b.chars().collect();

    let l1: usize = num1.len();
    let l2: usize = num2.len();
    let count = l1.max(l2);

    let mut carry: u32 = 0;
    let mut result = String::new();
    for c in 1..=count {
        let n1 = if l1 >= c { num1[l1 - c].to_digit(10).unwrap() } else { 0 };
        let n2 = if l2 >= c { num2[l2 - c].to_digit(10).unwrap() } else { 0 };
        let tmp = n1 + n2 + carry;
        carry = tmp / 2;
        result = format!("{}{}", (tmp%2), result);
    }
    if carry > 0 {
        result = format!("1{}", result);
    }
    return result;
}

// https://leetcode-cn.com/problems/zigzag-conversion/solution/
pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 { return s; }
    let mut cache = vec![String::new(); num_rows as usize];
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    let mut direction: i32 = -1;
    let mut irow: i32 = 0;
    for i in 0..len {
        cache[irow as usize].push(chars[i]);

        if i as i32 % (num_rows -1 ) == 0 {
            direction = -direction;
        }
        irow = irow + direction;
    }
    return cache.join("");
}

// https://leetcode-cn.com/problems/valid-number/solution/
pub fn is_number(s: String) -> bool {
    true
}

// what should be a number?
fn digit_transfer(chars: &Vec<char>, next: usize) -> bool {
    // if next == chars.len() {
    //     return true;
    // } else if chars[next].is_digit(10) {
    //     return digit_transfer(chars, next + 1);
    // } else if chars[next].
    true
}

// https://leetcode-cn.com/problems/ti-huan-kong-ge-lcof/
pub fn replace_space2(s: String) -> String {
    if s.is_empty() { return "".to_string(); }
    let len = s.len();
    let mut res = String::new();
    for c in s.chars() {
        if c == ' ' {
            res.push_str("%20");
        } else {
            res.push(c);
        }
    }
    return res;
}

pub fn replace_spaces(s: String, length: i32) -> String {
    if length == 0 { return String::new(); }

    let s = &s[..length as usize];
    let mut res = String::new();
    for c in s.chars() {
        if c == ' ' {
            res.push_str("%20");
        } else {
            res.push(c);
        }
    }
    return s.to_string();
}

// https://leetcode-cn.com/problems/di-yi-ge-zhi-chu-xian-yi-ci-de-zi-fu-lcof/
// 第一个只出现一次的字符
pub fn first_uniq_char(s: String) -> char {
    let mut record = vec![0; 26];
    let empty = ' ';
    let base = 'a'.to_digit(36).unwrap();
    for c in s.chars() {
        let n = c.to_digit(36).unwrap();
        record[(n - base) as usize] += 1;
    }

    for c in s.chars() {
        let n = c.to_digit(36).unwrap();
        if record[(n - base) as usize] == 1 {
            return c;
        }
    } 
    return empty;
}

// https://leetcode-cn.com/problems/length-of-last-word/
pub fn length_of_last_word(s: String) -> i32 {
    let mut res = String::new();
    let bytes = s.as_bytes();
    let empty = b' ';

    let mut q = bytes.len() as i32 - 1;
    let mut p = q;
    
    while q >= 0 {
        if bytes[q as usize] == empty {
            q -= 1;
        } else {
            if res.len() > 0 {
                res.push(empty as char);
            }
            p = q;
            while p >= 0 && bytes[p as usize] != empty {
                p -= 1;
            }
            return q - p;
        }
    }
    return 0;
}

// https://leetcode-cn.com/problems/fan-zhuan-dan-ci-shun-xu-lcof/comments/
// 反转的东西就是栈
pub fn reverse_words(s: String) -> String {
    let mut res = String::new();
    let bytes = s.as_bytes();
    let empty = b' ';

    let mut q = bytes.len() as i32 - 1;
    let mut p = q;
    
    while q >= 0 {
        if bytes[q as usize] == empty {
            q -= 1;
        } else {
            if res.len() > 0 {
                res.push(empty as char);
            }
            p = q;
            while p >= 0 && bytes[p as usize] != empty {
                p -= 1;
            }
            let start = (p + 1) as usize;
            let end = q as usize;
            let word = &s[start..=end];
            res.push_str(word);
            q = p - 1;
        }
    }
    return res;
}


// https://leetcode-cn.com/problems/zi-fu-chuan-de-pai-lie-lcof/comments/
// python
// class Solution:
//     def permutation(self, s: str) -> List[str]:
//         if not s: return 
//         s=list(sorted(s))
//         res=[]
//         def helper(s,tmp):
//             if not s: res.append(''.join(tmp))
//             for i,char in enumerate(s):
//                 if i>0 and s[i]==s[i-1]:
//                     continue
//                 helper(s[:i]+s[i+1:],tmp+[char])
//         helper(s,[])
//         return res
pub fn permutation(s: String) -> Vec<String> {
    // rust 令人沮丧的地方在于，一些直观的算法变得非常 messy
    Vec::new()
}

// https://leetcode-cn.com/problems/zuo-xuan-zhuan-zi-fu-chuan-lcof/comments/
// pub fn reverse_left_words(s: String, n: i32) -> String {
// }

// https://leetcode-cn.com/problems/valid-palindrome/
pub fn is_palindrome(s: String) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() <= 1 { return true; }

    let zero = b'0';
    let nine = b'9';
    let A = b'A';
    let Z = b'Z';
    let z = b'z';
    let a = b'a';
    // 大小写转换，来自https://leetcode-cn.com/problems/valid-palindrome/comments/
    let is_valid = |b| { ((b >= zero) && (b <= nine)) || ((b >= A ) && ( b <= Z)) || ((b >= a) && (b <= z))};
    let is_equal = |p, q| { p == q || (p & 0xDF) == q || p == (q & 0xDF)};
    let mut p = 0;
    let mut q = bytes.len() - 1;
    while q > p {
        while !is_valid(bytes[p]) && q > p {
            p += 1;
        }
        while !is_valid(bytes[q]) && q > p {
            q -= 1;
        }
        // dbg!(p, q, bytes[p], bytes[q]);
        if p == q { return true; }
        // now p q are valid
        else if is_equal(bytes[p], bytes[q]) {
            p += 1;
            q -= 1;
        } else {
            return false;
        }
    }
    return true;
}


fn main()
{
    // let s1 = "6913259244".to_string();
    // let s2 = "71103343".to_string();
    // let res = add_strings(s1, s2);
    // println!("result {}", res);
    // let s = "Leetcode".to_string();
    // println!("{}", convert(s, 2));
    // is_number("123".to_string());
    // replace_spaces("mory love jenny    x".to_string(), 15);
    // dbg!(replace_space2("".to_string()));
    // dbg!(first_uniq_char("abcdkefd".to_string()));
    // dbg!(reverse_words("Mory love Jenny! ".to_string()));
    // dbg!(reverse_words("  hello world!  ".to_string()));
    // permutation("dcdbvde".to_string());
    // reverse_left_words("hcade".to_string(), 2);
    // dbg!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    // dbg!(is_palindrome(",;".to_string()));
    // dbg!(is_palindrome("0P".to_string()));
    dbg!(is_palindrome("`l;`` 1o1 ??;l`".to_string()));
    
}