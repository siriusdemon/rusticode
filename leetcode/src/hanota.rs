// https://leetcode-cn.com/problems/hanota-lcci/
// 要求是 a -> c
// 思考：如何把 a 最下面的那块放到 c 上？
// 要完成这个，先完成 hanota 的子任务，把 a 除最下面的那些放到 C 上。
pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
    fn helper(n: usize, a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        if n == 1 {
            let x = a.pop().unwrap();
            c.push(x);
        } else {
            helper(n-1, a, c, b);
            helper(1, a, b, c);
            helper(n-1, b, a, c);
        }
    }
    helper(a.len(), a, b, c);
}


fn main()
{
    let mut a = vec![11, 32, 42];
    let mut b = vec![];
    let mut c = vec![];
    hanota(&mut a, &mut b, &mut c);
    println!("{:?}\n{:?}\n{:?}", a, b, c);
}

