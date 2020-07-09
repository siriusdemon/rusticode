// https://leetcode-cn.com/problems/n-queens/
// 皇后问题
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    /// n 网格大小，也是皇后的数目
    /// queens 表示已经定好的位置。如 [1, 4] 表示第一个皇后在0行1列，第二个皇后在1行4列
    /// res 用于缓存结果
    fn solver(n: usize, queens: Vec<usize>, res: &mut Vec<Vec<usize>>) {
        // 首先，明确自己是第几行的皇后
        let k = queens.len();
        // 其次，算出前面的皇后对我当前行的封锁。每个皇后对当前行都有三种封锁
        // 比如位于 i 行 j 列的皇后，对当前行 k 的封锁是这样计算的
        // row[j] = 1 同列封锁
        // let d = k - i
        // row[j-d] = 1 向后封锁（在不越界的情况下）
        // row[j+d] = 1 向前封锁（在不越界的情况下）
        let mut row = vec![0; n];
        for (i, &j) in queens.iter().enumerate() {
            row[j] = 1;
            let d = k - i;
            if j >= d { row[j-d] = 1; }
            if (j + d) < n { row[j+d] = 1; }
        }
        // 现在，封锁计算已经完成了。看看还有没有位置容得下本皇后，容得下就走，容不下就算了。
        for (i, &r) in row.iter().enumerate() {
            // 有空位就坐下
            if r == 0 {
                let mut new_queens = queens.clone();
                new_queens.push(i);
                // 如果8皇后坐好了，就回家
                if new_queens.len() == n {
                    res.push(new_queens);
                } else {
                    solver(n, new_queens, res);
                }
            }
        }
    } 
    fn formater(n: usize, res: Vec<Vec<usize>>) -> Vec<Vec<String>> {
        let mut formats: Vec<Vec<String>> = Vec::new();
        for queens in res {
            let mut format = Vec::new();
            for i in queens {
                // 格式化每一行。如果第 i 位是皇后，那么她前面有 i 个点（空位），她后面有 (n - 1 - i) 个点
                let mut bytes = vec!['.' as u8; n];
                bytes[i] = 'Q' as u8;
                let s = unsafe { String::from_utf8_unchecked(bytes) };
                format.push(s);
            }
            formats.push(format);
        }
        return formats;
    }
    let n = n as usize;
    let queens = vec![];
    let mut res = vec![];
    solver(n, queens, &mut res);
    // for r in &res {
    //     println!("{:?}", r);
    // }
    return formater(n, res);
}

fn main()
{
    let results = solve_n_queens(4);
    for res in results {
        println!("");
        for row in res {
            println!("{:?}", row);
        }
    }
}