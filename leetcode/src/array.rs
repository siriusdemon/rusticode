// https://leetcode-cn.com/problems/missing-number-lcci/
// ???
pub fn missing_number2(mut nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut expect_sum = n * (n + 1) / 2;
    for i in nums {
        expect_sum -= i;
    }
    return expect_sum;
}


// https://leetcode-cn.com/problems/merge-sorted-array/

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut k = m + n - 1;

    while i >= 0 && j >= 0 {
        if nums1[i as usize] >= nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }

    while j >= 0 {
        nums1[k as usize] = nums2[j as usize];
        k -= 1;
        j -= 1;
    }
}

// https://leetcode-cn.com/problems/diao-zheng-shu-zu-shun-xu-shi-qi-shu-wei-yu-ou-shu-qian-mian-lcof/
// 奇数前进，偶数后退，用快排思路
pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 0 { return nums; }


    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut nums = nums;

    while right > left {
        while (right > left) && (nums[left] % 2 == 1) {
            left += 1;
        }
        
        while (right > left) && (nums[right] % 2 == 0) {
            right -= 1;
        }
        if right > left {
            let t = nums[left];
            nums[left] = nums[right];
            nums[right] = t; 
        }
    }
    return nums;
}



pub fn missing_number(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut p = 0;
    let mut q = len;

    while p < q {
        let mid = (p + q) / 2;
        if nums[mid] == mid as i32 {
            p = mid + 1;
        } else {
            q = mid;
        }
    }
    return p as i32;
}


// https://leetcode-cn.com/problems/shu-zu-zhong-chu-xian-ci-shu-chao-guo-yi-ban-de-shu-zi-lcof/
// 数字超过一半，说明中间的数一定是那个数
pub fn majority_element(nums: Vec<i32>) -> i32 {
    fn parition(nums: &mut Vec<i32>, p: usize, q :usize) -> usize {
        let last = q - 1;
        let pivot = nums[last];
        let mut p = p;
        let mut q = last - 1;
        while p < q {
            if nums[p] >= pivot {
                let t = nums[p];
                nums[p] = nums[q];
                nums[q] = t;
                q -= 1;
            } else {
                p += 1;
            }
        }
        // p == q
        if nums[p] >= pivot {
            nums[last] = nums[p];
            nums[p] = pivot;
        } else {
            p += 1;
            nums[last] = nums[p];
            nums[p] = pivot;
        }
        return p;
    }
    fn quicksearch(nums: &mut Vec<i32>, p: usize, q: usize, k: usize) -> i32 {
        if q > (1 + p) {
            let ind = parition(nums, p, q);
            if ind == k {
                return nums[ind];
            } else if ind < k {
                return quicksearch(nums, ind+1, q, k);
            } else {
                return quicksearch(nums, p, ind, k);
            }
        } else {
            return nums[p];
        }
    }
    let mut nums = nums;
    let len = nums.len();
    return quicksearch(&mut nums, 0, len, len / 2);
}

// https://leetcode-cn.com/problems/shu-zu-zhong-chu-xian-ci-shu-chao-guo-yi-ban-de-shu-zi-lcof/
pub fn majority_element2(nums: Vec<i32>) -> i32 {
    // 由于超过一半，所以用摩尔投票法
    let mut c = 0;
    let mut card: i32 = 0;
    let len = nums.len();
    for i in 0..len {
        if c == 0 {
            card = nums[i];
            c += 1;
        }
        else if card == nums[i] {
            c += 1;
        } else {
            c -= 1;
        }
    }
    return card;
}

// https://leetcode-cn.com/problems/he-wei-sde-liang-ge-shu-zi-lcof/comments/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut p = 0;
    let mut q = nums.len() - 1;

    while q > p {
        let s = nums[p] + nums[q];
        if s == target {
            return [nums[p], nums[q]].to_vec();
        } else if s < target {
            p += 1;
        } else {
            q -= 1;
        }
    }
    return [].to_vec();
}


// https://leetcode-cn.com/problems/daily-temperatures/solution/
// 递减栈
pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let len = t.len();
    let mut res: Vec<i32> = vec![0; len];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..len {
        if !stack.is_empty() {
            let mut last_i = stack[stack.len()-1];
            while t[last_i] < t[i] {
                res[last_i] = (i - last_i) as i32;
                stack.pop();
                if stack.is_empty() {
                    break;
                } else {
                    last_i = stack[stack.len()-1];
                }
            }
        }
        stack.push(i);
    }
    return res;
}

// https://leetcode-cn.com/problems/xuan-zhuan-shu-zu-de-zui-xiao-shu-zi-lcof/solution/mian-shi-ti-11-xuan-zhuan-shu-zu-de-zui-xiao-shu-3/
// 可以二边的题，必有二分的条件：
// 两个递增数组合在一起，最小值就是后面一个数组的第一个数
pub fn min_array(numbers: Vec<i32>) -> i32 {
    let mut p = 0;
    let mut q = numbers.len() - 1;

    while q > p {
        let m = (p + q) / 2;
        // m 一定在左边的数组中
        if numbers[m] > numbers[q] {
            p = m + 1;
        // m 一定落在右边的数组中
        } else if numbers[m] < numbers[q] {
            q = m;
        // 无法判断落在左边还是右边，缩小范围
        } else {
            q -= 1;
        }
    }
    return numbers[p];
}

// https://leetcode-cn.com/problems/yuan-quan-zhong-zui-hou-sheng-xia-de-shu-zi-lcof/comments/
// 约瑟夫环
pub fn last_remaining(n: i32, m: i32) -> i32 {
    42
}

// https://leetcode-cn.com/problems/best-sightseeing-pair/solution/python-jie-fa-by-jiayangwu/
pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut pre = a[0] + 0;
    for j in 1..a.len() {
        // 这一行是计算最大值
        res = res.max(pre + a[j] - j as i32);
        // 这一行是更新最高的
        pre = pre.max(a[j] + j as i32);
    }
    return res;
}

// https://leetcode-cn.com/problems/lian-xu-zi-shu-zu-de-zui-da-he-lcof/
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut sum = nums[0];
    let mut res = nums[0];
    for i in 1..nums.len() {
        if sum > 0 {
            sum += nums[i];
        } else {
            sum = nums[i];
        }
        res = sum.max(res);
    }
    return res;
}


// https://leetcode-cn.com/problems/qing-wa-tiao-tai-jie-wen-ti-lcof/
pub fn num_ways(n: i32) -> i32 {
    if n == 0 { return 1; }
    else if n == 1 { return 1; }
    else if n == 2 { return 2; }
    else {
        // 现在想计算 3 
        let mut a = 1;
        let mut b = 2;
        let mut res = 0;
        let m = 1000000007;
        for i in 2..n {
            res = a + b ;
            a = b;
            b = res % m;
        }
        return res % m;
    }
}


// https://leetcode-cn.com/problems/hua-dong-chuang-kou-de-zui-da-zhi-lcof/
// pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
// }

// https://leetcode-cn.com/problems/gou-jian-cheng-ji-shu-zu-lcof/solution/c-on-jie-fa-by-yizhe-shi/
pub fn construct_arr(mut a: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut res = vec![0; n];
    let mut left = 1;
    // 左侧有累加的形式。
    // 如 a[n-1] 跟 a[n] 相比，a[n] 的左侧积是 a[n-1] 的 a[i] 倍
    for i in 0..n {
        res[i] = left;
        left = left * a[i];
    }
    let mut right = 1;
    for i in (0..n).rev() {
        res[i] *= right;
        right *= a[i];
    } 
    return res;
}


// https://leetcode-cn.com/problems/shu-zu-zhong-shu-zi-chu-xian-de-ci-shu-ii-lcof/
// solution/mian-shi-ti-56-ii-shu-zu-zhong-shu-zi-chu-xian-d-4/
// 二进制模拟三进制，自动有限状态机
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut t0 = 0;
    let mut t1 = 0;

    for n in nums {
        t0 = t0 ^ n & !t1; 
        t1 = t1 ^ n & !t0; 
    }
    return t0;
}


// https://leetcode-cn.com/problems/bu-ke-pai-zhong-de-shun-zi-lcof/
// 是不是顺子，这是一个奇怪的问题
// 还不能重复
pub fn is_straight(mut nums: Vec<i32>) -> bool {
    let mut max_ = 1;
    let mut min_ = 15;
    let mut zc = 0;
    let mut dup = vec![0; 15];
    for n in nums {
        if n == 0 {
            zc += 1;
        } else {
            if dup[n as usize] == 1 {
                return false;
            } 
            dup[n as usize] = 1;
            max_ = max_.max(n);
            min_ = min_.min(n);
        }
    }
    return (max_ - min_) <= (zc + 4);
}

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort();
    let n = nums.len();
    let mut ans = nums[0] + nums[1] + nums[2];
    for i in 0..n-2 {
        let mut start = i + 1;
        let mut end = n - 1;
        while start < end {
            let sum = nums[i] + nums[start] + nums[end];
            if (sum - target).abs() < (ans - target).abs() {
                ans = sum;
            } else if sum > target {
                end -= 1;
            } else if sum < target {
                start += 1;
            } else {
                return ans;
            }
        }
    }
    return ans;
}

// https://leetcode-cn.com/problems/rotate-array/
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    
}

// https://leetcode-cn.com/problems/ju-zhen-zhong-de-lu-jing-lcof/
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    true
}


// https://leetcode-cn.com/problems/shu-zu-zhong-de-ni-xu-dui-lcof/comments/
pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
    32
}

// https://leetcode-cn.com/problems/rotate-matrix-lcci/comments/
pub fn rotate2(matrix: &mut Vec<Vec<i32>>) {
    let row = matrix.len();
    let col = matrix[0].len();

    for i in 0..row {
        for j in i+1..col {
            let t = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = t;
        }
    }
    for i in 0..row {
        for k in 0..col/2 {
            let t = matrix[i][k];
            matrix[i][k] = matrix[i][col-k-1];
            matrix[i][col-k-1] = t;
        }    
    }
}

// https://leetcode-cn.com/problems/magic-index-lcci/submissions/
pub fn find_magic_index(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    while i < nums.len() {
        if nums[i] == i as i32 {
            return nums[i];
        } else if nums[i] > i as i32 {
            i = nums[i] as usize;
        } else {
            i += 1;
        }
    }
    return -1;
}

// https://leetcode-cn.com/problems/zero-matrix-lcci/
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    fn set_row_zeros(matrix: &mut Vec<Vec<i32>>, nrow: usize, col: usize) {
        for j in 0..col {
            matrix[nrow][j] = 0;
        }
    }
    fn set_col_zeros(matrix: &mut Vec<Vec<i32>>, ncol: usize, row: usize) {
        for i in 0..row {
            matrix[i][ncol] = 0;
        }
    }
    let row = matrix.len();
    let col = matrix[0].len();
    // first row 
    let mut row0 = 1;
    let mut col0 = 1;
    for i in 0..col {
        if matrix[0][i] == 0 {
            row0 = 0;
            break;
        }
    }
    for i in 0..row {
        if matrix[i][0] == 0 {
            col0 = 0;
            break;
        }
    }
    // record other row and column
    for i in 1..row {
        for j in 1..col {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }
    // handle row and column except the 1st
    for i in 1..row {
        if matrix[i][0] == 0 {
            set_row_zeros(matrix, i, col);
        }
    } 
    for j in 1..col {
        if matrix[0][j] == 0 {
            set_col_zeros(matrix, j, row);
        }
    }
    if row0 == 0 {
        set_row_zeros(matrix, 0, col);
    }
    if col0 == 0 {
        set_col_zeros(matrix, 0, row)
    }
}

fn main()
{
    // dbg!(majority_element([1].to_vec()));
    // dbg!(majority_element([1, 1].to_vec()));
    // dbg!(majority_element([1, 2, 2].to_vec()));
    // dbg!(majority_element([1, 2, 2, 2].to_vec()));
    // dbg!(majority_element([1,2,3,2,2,2,5,4,2].to_vec()));
    // let x = [47,47,72,47,72,47,79,47,12,92,13,47,47,83,33,15,18,47,47,47,47,64,47,65,47,47,47,47,70,47,47,55,47,15,60,47,47,47,47,47,46,30,58,59,47,47,47,47,47,90,64,37,20,47,100,84,47,47,47,47,47,89,47,36,47,60,47,18,47,34,47,47,47,47,47,22,47,54,30,11,47,47,86,47,55,40,49,34,19,67,16,47,36,47,41,19,80,47,47,27].to_vec();
    // dbg!(majority_element2(x));
    // dbg!(daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73].to_vec()));
    // dbg!(min_array([ 1,3,5].to_vec()));
    // dbg!(min_array([2, 2, 2, 0, 1].to_vec()));
    // dbg!(min_array([4,5,6,2,2,2,].to_vec()));
    // dbg!(min_array([3, 3, 1, 3].to_vec()));
    // dbg!(min_array([10,10,10,1,10].to_vec()));
    // dbg!(construct_arr([1,2,3,4].to_vec()));
    // dbg!(single_number([3, 4, 3, 3].to_vec()));
    // dbg!(is_straight([1,2,3,4,5].to_vec()));
    // dbg!(is_straight([1,0,3,0,5].to_vec()));
    // dbg!(is_straight([1,2,3,4,6].to_vec()));
    // dbg!(is_straight([0,0,2,2,5].to_vec()));
    // three_sum_closest([3,4,5,6,7,2,3,4,5,7].to_vec(), 3);
    // let mut matrix = vec![
    //     [1,2,0].to_vec(),
    //     [4,5,6].to_vec(),
    //     [7,8,9].to_vec(),
    // ];
    // set_zeroes(&mut matrix);
    // println!("{:?}", matrix);
    missing_number2([9,6,4,2,3,5,7,0,1].to_vec());
}