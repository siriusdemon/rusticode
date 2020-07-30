// https://leetcode-cn.com/problems/hanota-lcci/
// 要求是 a -> c
// 思考：如何把 a 最下面的那块放到 c 上？
// 要完成这个，先完成 hanota 的子任务，把 a 除最下面的那些放到 C 上。
pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
    fn helper(n: usize, a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        if n > 1 { helper(n-1, a, b, c); } 
        helper(c.len(), c, a, b); // C -> B
        c.push(a[n]);
        a.pop();
        helper(b.len(), b, a, c); // B -> C`
    }
    let an = a.len();
    helper(an, a, b, c); // A -> C
}


fn main()
{
    let mut a = vec![1, 0];
    let mut b = vec![];
    let mut c = vec![];
    hanota(&mut a, &mut b, &mut c);
    println!("{:?}\n {:?}\n {:?}", a, b, c);
}

