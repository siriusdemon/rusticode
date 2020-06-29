// https://leetcode-cn.com/problems/subarray-sums-divisible-by-k/solution/
// 给定一个整数数组 A，返回其中元素之和可被 K 整除的（连续、非空）子数组的数目。
// 暴力法
pub fn force_subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
    let len = a.len();
    // 1 more place, sum[0] === 0
    let mut sum = vec![0; len+1];
    let mut count = 0;
    
    for i in 0..len {
        // sum[i] last presum, a[i], current value => current sum
        let s = sum[i] + a[i];
        for j in 0..=i {
            let subsum = s - sum[j];
            // println!("{}", subsum);
            if subsum % k == 0 {
                count += 1;
            }
        }
        // current sum
        sum[i+1] = s;
    }
    return count;
}

// 两数之差能被k整除，则说明这两数余k相同。
// https://leetcode-cn.com/problems/subarray-sums-divisible-by-k/comments/
// 举例 对于A= [4,5,0,-2,-3,1] K = 5，
// s = [0, 4, 9, 9, 7, 4, 5] ，
// kcnt = [2, 0, 1, 0, 4] 代表有s中有两个元素的余数都为0（即0和5），1个元素的余数为2（即7），四个元素的余数为4（即4994）
// 所以在保证余数相同的情况下，取出两个数都可以得到一组答案。对于这个例子答案就是 C22 + C12 + C42 = 1 + 0 + 6 = 7
pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
    let len = a.len();
    let mut sum = vec![0; len+1];
    let remlen = k as usize;
    let mut remlist = vec![0; remlen];

    for i in 0..len {
        sum[i+1] = (a[i] % k) + k + sum[i];
    }
    println!("{:?}", sum);

    for i in 0..=len {
        let rem = sum[i] % k;
        remlist[rem as usize] += 1; 
    }
    println!("{:?}", remlist);

    let mut count = 0;
    for i in 0..remlen {
        let times = remlist[i];
        if times > 1 {
            count += times * (times -1) / 2;
        }
    }
    return count;
}

fn main()
{
    // let a = [4,5,0,-2,-3,1].to_vec();
    // let a = [-2].to_vec();
    let a = [-6,1,-5,10].to_vec();
    
    let x = subarrays_div_by_k(a, 6);
    println!("{}", x);
}