// https://leetcode-cn.com/problems/shu-zu-zhong-zhong-fu-de-shu-zi-lcof/
// 鹊巢原理，把本来该是我的位置还给我。同时是一种无重复数字的排序算法。
// 坐车换票算法，因为每次都会至少换好一个位置，所以只需要换 n 次
pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
    let mut nums = nums;

    for i in 0..nums.len() {
        // while 循环每次结束，都把第 i 个位置上的数摆正了
        while nums[i] != i as i32 {
            // 现在，本该是 i 的位置被 next 占了
            let next = nums[i];
            // 先把 next 放在他的位置上，但如果 pnext 已经等于 next 了，就说明 next 重复
            let pnext = nums[next as usize];
            if pnext == next {
                return pnext;
            }
            nums[next as usize] = next;   
            // 先暂时让 pnext 坐在 i 的位置上
            nums[i] = pnext;
        }
    }
    return -1;
}


// https://leetcode-cn.com/problems/er-wei-shu-zu-zhong-de-cha-zhao-lcof/comments/
// 技术性题目，从右上角开始，左边的都是小数，右边的都是大数
// 对于这道题，使用 i32 加类型转换会更简洁
pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let row = matrix.len();
    if row == 0 { return false; }
    let col = matrix[0].len();
    if col == 0 { return false; }

    let mut i = 0;
    let mut j = col as i32 - 1;
    while i < row && j >= 0 {
        let p = matrix[i][j as usize];
        if p < target {
            i += 1;
        } else if p > target {
            j -= 1;
        } else {
            return true;
        }
    }
    return false;
}


// https://leetcode-cn.com/problems/da-yin-cong-1dao-zui-da-de-nwei-shu-lcof/solution/
// 原意，用字符串模拟数学，避免大数相加时溢出
pub fn print_numbers(n: i32) -> Vec<i32> {
    Vec::new()
}


// https://leetcode-cn.com/problems/jian-sheng-zi-lcof/
// 进行数学分析，若令 x 为绳子切出来的一小段
// 因为 3 * (x - 3) > x 在 x > 5 时成立，所以 x < 5
// 因为 4 = 2 * 2，所以 x = 4 时相当于 x = 2
// 因为 2 * 2 * 2 < 3 * 3，所以 2 的段数不应该超过 2 个
pub fn cutting_rope(mut n: i32) -> i32 {
    if n <= 3 { return n - 1; }
    let mut res = 1;
    // 一个 2
    if n % 3 == 2 {
        n -= 2;
        res *= 2;
    }
    // 两个 2
    if n % 3 == 1 {
        n -= 4;
        res *= 4;
    }

    while n > 0 {
        n -= 3;
        res *= 3;
    }
    return res;
}

// https://leetcode-cn.com/problems/shun-shi-zhen-da-yin-ju-zhen-lcof/
// 顺时针打印矩阵
// buggy code without fix
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let row = matrix.len();
    if row == 0 { return [].to_vec(); }
    let col = matrix[0].len();

    let mut res = vec![0; col*row];
    let mut k = 0;
    let mut i = 0;
    let mut j = 0;
    let mut rightB = col;
    let mut downB = row;
    let mut upB = 0;
    let mut leftB = 0;

    while k < col*row {
        // start to right，最后一个不要吃
        while j < rightB-1 {
            res[k] = matrix[i][j];
            j += 1;
            k += 1;
            println!("{:?}", res);
        }
        // start down
        while i < downB-1 {
            res[k] = matrix[i][j];
            i += 1;
            k += 1;
            println!("{:?}", res);
        }
        // start left
        while j > leftB {
            res[k] = matrix[i][j];
            j -= 1;
            k += 1;
            println!("{:?}", res);
        }
        // start up
        while i > upB {
            res[k] = matrix[i][j];
            i -= 1;
            k += 1;
            println!("{:?}", res);
        }
        // 这样第一圈结束了，更新边界
        upB = upB + 1;
        rightB = rightB - 1;
        downB = downB - 1;
        leftB = leftB + 1;
        j = j + 1;
        i = i + 1;
    }
    return res;
}


// https://leetcode-cn.com/problems/ji-qi-ren-de-yun-dong-fan-wei-lcof/
// 数位之和，采用广度优先搜索算法
// 其中的核心是表 matrix
fn int_bit_sum(mut n: i32) -> i32 {
    let mut s = 0;
    while n > 0 {
        s += n % 10;
        n = n / 10;
    }
    return s;
}

fn dfs(matrix: &mut Vec<Vec<i32>>, i: i32, j: i32, k: i32, m: i32, n: i32) -> i32 {
    if (i < 0) || (i >= m) || (j < 0) || (j >= n) { return 0; }
    if (int_bit_sum(i) + int_bit_sum(j)) > k { return 0; }
    if matrix[i as usize][j as usize] == 1 { return 0; }
    matrix[i as usize][j as usize] = 1;
    let down = dfs(matrix, i+1, j, k, m, n); 
    let right = dfs(matrix, i, j+1, k, m, n);
    return down + right + 1;
}

pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
    let mut matrix = vec![vec![0; n as usize]; m as usize]; // m x n
    let c = dfs(&mut matrix, 0, 0, k, m, n);
    return c;
}

// https://leetcode-cn.com/problems/fei-bo-na-qi-shu-lie-lcof/
pub fn fib(n: i32) -> i32 {
    if n == 0 { return 0; }
    else if n == 1 { return 1; }
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let t = a;
        a = b;
        b = (t + a) % 1000000007;
    }
    return b;
}

fn main()
{
    // let x = find_repeat_number([1, 2, 3, 2, 4].to_vec());
    // let x = find_repeat_number([4, 0, 3, 1, 2, 1, 5, 9, 7, 8].to_vec());
    // let matrix = [
    //     [1,2,3].to_vec(),
    //     [5,6,7].to_vec(),
    //     [9,8,10].to_vec(),
    // ].to_vec();
    // // let matrix = [[1].to_vec()].to_vec();
    // let x = spiral_order(matrix);
    // println!("{:?}", x);
    // dbg!(moving_count(11, 8, 16));
    // dbg!(moving_count(16, 8, 4));
    for n in 0..10 {
        println!("{}", fib(n));
    }
}

