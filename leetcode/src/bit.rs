// https://leetcode-cn.com/problems/bu-yong-jia-jian-cheng-chu-zuo-jia-fa-lcof/solution/
pub fn add(a: i32, b: i32) -> i32 {
    if b == 0 { return a; }
    if a == 0 { return b; }
    let a1 = (a ^ b);
    let b1 = ((a & b) << 1);
    return add(a1, b1);
}


// https://leetcode-cn.com/problems/shu-zu-zhong-shu-zi-chu-xian-de-ci-shu-lcof/
// 一个数组中，只有两个数不重复，其他数都重复了两次。
// 两次异或等于原数，可以用异或来区分
// 用 异或后的 1位，来分组，将这个数组分成两个数组，分别异或，可以得到两个数
pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut res = 0;
    for n in &nums {
        res = res ^ n;
    }
    let mut mask = 1;
    while (mask & res) == 0 {
        mask = mask << 1;
    }

    let mut one = 0;
    let mut two = 0;
    for n in &nums {
        if n & mask == 0 {
            one = one ^ n;
        } else {
            two = two ^ n;
        }
    }
    return [one, two].to_vec();
}


fn main()
{
    dbg!(add(10, 9));
    dbg!(add(-3, 10));
}