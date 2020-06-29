// https://leetcode-cn.com/problems/powx-n/comments/
pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 1 || x == 1.0 || x == 0.0 {
        return x;
    } else if n < 0 {
        // rust code to handle overflow
        let x = 1.0 / x;
        let nn = -(n / 2);
        if n % 2 == 0 {
            return x * my_pow(x * x, nn); 
        } else {
            return x * my_pow(x * x, nn); 
        }
    } else if n == 0 {
        return 1.0;
    } 
    if n % 2 == 0 {
        return my_pow(x * x, n / 2);
    } else {
        return x * my_pow(x * x, n / 2);
    }
}

// https://leetcode-cn.com/problems/jian-sheng-zi-ii-lcof/solution/
pub fn cutting_rope(mut n: i32) -> i32 {
    if n <= 3 { return n-1; }
    let m = 1000_000_000 + 7;
    let mut res: u32 = 1;
    while n > 4 {
        res *= 3;
        res %= m;
        n -= 3;
    }
    return (res * n % m) as i32;
}

fn main()
{
    let x: f64 = 2.0;
    let n = -2147483648;
    let r = my_pow(x, n);
    println!("{}", r);
}