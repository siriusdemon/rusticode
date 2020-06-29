pub fn nth_ugly_number(n: i32) -> i32 {
    let (mut p2, mut p3, mut p5) = (0, 0, 0);
    let mut cache: Vec<i32> = vec![1];
    let mut res = 1;
    for _ in 1..n {
        if cache[p2] * 2 == res { p2 += 1; }  // 重复的检查与指针的移动
        if cache[p3] * 3 == res { p3 += 1; }
        if cache[p5] * 5 == res { p5 += 1; }
        res = (2 * cache[p2]).min(3 * cache[p3]).min(5 * cache[p5]);
        cache.push(res);
    }
    return res;
}

fn main()
{
    let n = 11;
    for i in 0..20 {
        println!("{}th ugly number {}", i+1, nth_ugly_number(i+1));
    }
}