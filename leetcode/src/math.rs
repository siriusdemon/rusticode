// https://leetcode-cn.com/problems/powx-n/comments/
pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 1 { return x; }
    if n == -1 { return 1 / x; }
    if n == 0 { return 1; }
    let half = my_pow(x, n / 2);
    let mod_ = my_pow(x, n % 2);
    return half * half * mod_;
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


// https://leetcode-cn.com/problems/diving-board-lcci/solution/
pub fn diving_board(shorter: i32, longer: i32, k: i32) -> Vec<i32> {
    if k == 0 { return vec![]; }
    if shorter == longer { return vec![shorter * k]; }
    let k = k as usize;
    let mut res = vec![0; k+1];
    for i in 0..=k {
        res[i] = shorter * (k - i) as i32 + longer * i as i32;
    }
    return res;
}


// https://leetcode-cn.com/problems/shu-zi-xu-lie-zhong-mou-yi-wei-de-shu-zi-lcof/
pub fn find_nth_digit(n: i32) -> i32 {

}

fn main()
{
    let x: f64 = 2.0;
    let n = -2147483648;
    let r = my_pow(x, n);
    println!("{}", r);
}