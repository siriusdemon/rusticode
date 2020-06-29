// 给定高度m 、宽度n 的一张 m * n的乘法表，以及正整数k，你需要返回表中第k 小的数字。
// 1   2   3
// 2   4   6
// 3   6   9
// 4   8   12

// n, m, k >= 1
// m = 4 , n = 3
pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
    42
}

// https://leetcode-cn.com/problems/kth-largest-element-in-an-array/
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    fn partition(nums: &mut Vec<i32>, mut p: usize, mut q: usize) -> usize {
        let last = q - 1;
        let pivot = nums[last];
        let mut q = last - 1;
        while p < q {
            if nums[p] < pivot {
                let t = nums[p];
                nums[p] = nums[q];
                nums[q] = t;
                q -= 1;
            } else {
                p += 1;
            }
        }
        // p == q
        if nums[p] < pivot {
            nums[last] = nums[p];
            nums[p] = pivot;
        } else {
            p += 1;
            nums[last] = nums[p];
            nums[p] = pivot;
        }
        return p;
    }
    fn quicksort(nums: &mut Vec<i32>, k: i32, p: usize, q: usize) -> i32 {
        if q > p + 1 {
            let mid = partition(nums, p, q);
            if mid+1 == k { return nums[mid]; }
            else if mid+1 < k {
                return quicksort(nums, k, mid+1, q);
            } else {
                return quicksort(nums, k, p, mid);
            }
        }
    }
    return quicksort(&nums, k, 0, nums.len());
}

// https://leetcode-cn.com/problems/get-kth-magic-number-lcci/
// 合并丑数序列
// 1 3 5 7
pub fn get_kth_magic_number(k: i32) -> i32 {
    let k = k as usize;
    let mut ugly: Vec<i32> = vec![0; k];
    let mut p3 = 0;
    let mut p5 = 0;
    let mut p7 = 0;
    
    ugly[0] = 1;
    for i in 1..k {
        ugly[i] = (ugly[p3] * 3).min(ugly[p5] * 5).min(ugly[p7] * 7);
        if ugly[i] == ugly[p3] * 3 { p3 += 1; }
        if ugly[i] == ugly[p5] * 5 { p5 += 1; }
        if ugly[i] == ugly[p7] * 7 { p7 += 1; }
    }
    return ugly[k - 1];
}

fn main()
{
    let x = get_kth_magic_number(9);
    println!("{}", x);
}