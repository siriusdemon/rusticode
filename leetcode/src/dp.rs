// 我们开始学习动态规划吧
use std::cmp::min;


// https://leetcode-cn.com/problems/maximum-subarray
// 最大子序各，好像看不出什么动态规则的意味，反而像滑动窗口
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut sum = nums[0];
    let mut ans = nums[0];

    for i in 1..nums.len() {
        if sum > 0 {
            // add positive sum means larger
            sum += nums[i];
        } else {
            // start from new one means larger
            sum = nums[i];
        }
        // ans always store the largest sum
        ans = std::cmp::max(sum, ans);
    }    
    ans
}


// https://leetcode-cn.com/problems/climbing-stairs/solution/
// basic dynamic programming
pub fn climb_stairs(n: i32) -> i32 {
    if n == 0 || n == 1 { 
        return 1;
    }

    // f(n) = f(n-1) + f(n-2)
    // iterative is harder than recursive
    let mut n_1 = 1; // f(n-1)
    let mut n_2 = 1; // f(n-2)
    let mut ans = 0;
    for _ in 1..n {
        ans = n_1 + n_2;
        n_1 = n_2;
        n_2 = ans; 
    }
    ans
}

// https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/solution/yi-ge-fang-fa-tuan-mie-6-dao-gu-piao-wen-ti-by-l-3/
// sell stock using state machine
// this is the solution for infinite k
pub fn max_profit_infinite(prices: Vec<i32>) -> i32 {
    let mut s_keep = std::i32::MIN; // you could not keep any stock on the very first day
    let mut s_empty = 0;

    for price in prices {
        s_keep = std::cmp::max(s_keep, s_empty - price); 
        s_empty = std::cmp::max(s_empty, s_keep + price);
    }
    return s_empty;
}

// https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/solution/zhuang-tai-ji-mo-xing-dp-by-acw_wangdh15/
// 用有限状态机的方式去解题
use std::i32;
pub fn max_profit_cool(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut dp = vec![vec![i32::MIN; 3]; n+1];
    // 0 可以买入的状态，买入之后转移到状态1。可以原地保持状态，或者从冷冻态转过来
    // 1 可以卖出的状态，卖出之后转移到状态2。可以原地保持状态，或者从状态0转过来
    // 2 冷冻期，过了一天转入状态0。可以从状态1转过来。

    // 0 明天可买入，要么今天不买，要么今天是冷冻期
    // 1 明天可卖出：要么今天买，要么今天不卖
    // 2 明天是冷冻，那就今天卖了吧
    dp[0][0] = 0;
    for i in 0..n {
        dp[i+1][0] = dp[i][0].max(dp[i][2]); // 来自 0 和 2 的转移
        dp[i+1][1] = dp[i][1].max(dp[i][0] - prices[i]);
        dp[i+1][2] = dp[i][1] + prices[i]; 
        // println!("dp[i][0]: {}", dp[i][0]);
        // println!("dp[i][1]: {}", dp[i][1]);
        // println!("dp[i][2]: {}", dp[i][2]);
    }    
    return dp[n][0].max(dp[n][2]);
}

pub fn max_profit_once(prices: Vec<i32>) -> i32 {
    // suffix 0 means no trade (buy or sell) happen
    // 1 means it happend
    // let mut s_keep_0 = std::i32::MIN; // you could not keep any stock on the very first day
    let mut s_empty_0 = 0; 
    let mut s_keep_1 = std::i32::MIN; 
    let mut s_empty_1 = std::i32::MIN;

    for price in prices {
        s_keep_1 = std::cmp::max(s_keep_1, s_empty_0 - price); 
        s_empty_1 = std::cmp::max(s_empty_1, s_keep_1 + price);
    }
    return std::cmp::max(s_empty_1, 0);
}

pub fn max_profit_twice(prices: Vec<i32>) -> i32 {
    // suffix 0 means no trade (buy or sell) happen
    // 1 means it happend
    // let mut s_keep_0 = std::i32::MIN; // you could not keep any stock on the very first day
    let mut s_empty_0 = 0; 
    let mut s_keep_1 = std::i32::MIN; 
    let mut s_empty_1 = std::i32::MIN;
    let mut s_keep_2 = std::i32::MIN;
    let mut s_empty_2 = std::i32::MIN;

    for price in prices {
        s_keep_1 = std::cmp::max(s_keep_1, s_empty_0 - price); 
        s_empty_1 = std::cmp::max(s_empty_1, s_keep_1 + price);
        s_keep_2 = std::cmp::max(s_keep_2, s_empty_1 - price);
        s_empty_2 = std::cmp::max(s_empty_2, s_keep_2 + price);
    }
    return std::cmp::max(s_empty_2, 0);
}


// this one works but consume too much memory
pub fn max_profit_k_memory_consume(k: i32, prices: Vec<i32>) -> i32 {
    // from example above, we know the initial value is 0
    // here, k become a variable, some we need a matrix to
    // store different status
    // how many status we have?
    // empty or keep => 2
    // trade times => k
    // so we have 2k status
    let mut s_trade: [i32; 2] = [std::i32::MIN, std::i32::MIN]; // trade state: empty or keep
    let mut s_times: Vec<[i32;2]> = Vec::new(); 
    let k: usize = k as usize;
    for i in 0..k+1 {
        s_times.push(s_trade.clone());
    }
    s_times[0][0] = 0;
    for price in prices {
        for j in 0..k {
            s_times[j+1][1] = std::cmp::max(s_times[j+1][1], s_times[j][0] - price);
            s_times[j+1][0] = std::cmp::max(s_times[j+1][0], s_times[j+1][1] + price);
        }
    }
    return std::cmp::max(0, s_times[k][0]);        
}

// memory efficient version
pub fn max_profit_k(k: i32, prices: Vec<i32>) -> i32 {
    // here if k in unreasonable large, switch to infinite version
    let k: usize = k as usize;
    if k > prices.len()/2 {
        return max_profit_infinite(prices);
    }

    let mut s_trade: [i32; 2] = [std::i32::MIN, std::i32::MIN]; // trade state: empty or keep
    let mut s_times: Vec<[i32;2]> = Vec::new(); 
    for i in 0..k+1 {
        s_times.push(s_trade.clone());
    }
    s_times[0][0] = 0;
    for price in prices {
        for j in 0..k {
            s_times[j+1][1] = std::cmp::max(s_times[j+1][1], s_times[j][0] - price);
            s_times[j+1][0] = std::cmp::max(s_times[j+1][0], s_times[j+1][1] + price);
        }
    }
    return std::cmp::max(0, s_times[k][0]);        
}

// shortest path
// https://leetcode-cn.com/problems/minimum-path-sum/
// way: set grid value as the cost to get there
// matrix: 
// 1  0  1      1  1  2
// 2  3  5  =>  3  4  7
// 5  3  2      8  7  9
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let row = grid.len();
    let col = grid[0].len();
    let mut cost = grid.clone();
    for r in 0..row {
        for c in 0..col {
            if r == 0 && c == 0 {
                cost[r][c] = grid[r][c];
            } else if r == 0 {
                cost[r][c] = grid[r][c] + cost[r][c-1];
            } else if c == 0 {
                cost[r][c] = grid[r][c] + cost[r-1][c];
            } else {
                cost[r][c] = grid[r][c] + min(cost[r-1][c], cost[r][c-1]);
            }
        }
    }
    return cost[row-1][col-1];
}


// https://leetcode-cn.com/problems/generate-parentheses/solution/
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    if n == 0 {
        return Vec::new();
    }

    let mut dp = vec![Vec::<String>::new(); (n+1) as usize];
    dp[0] = vec![String::from("")];
    for i in 1..=n {
        println!("Round {}", i);
        let mut cur = vec![];
        for j in 0..i {
            let left = &dp[j as usize];
            let right = &dp[(i-j-1) as usize];
            for l in left {
                for r in right {
                    let tmp = format!("({}){}", l, r);
                    println!("new string {}", tmp);
                    cur.push(tmp);
                }
            }
        }
        dp[i as usize] = cur;
    }

    let res = dp.pop().unwrap();
    return res
}

// https://leetcode-cn.com/problems/unique-paths/
// 到达P[i][j]的路径数 = P[i-1][j] + P[i][j-1]
pub fn unique_paths(m: i32, n: i32) -> i32 {
    if m == 1 || n == 1 {
        return 1;
    } else {
        return unique_paths(m - 1, n) + unique_paths(m, n - 1);
    }
}

pub fn unique_paths_iter(m: i32, n: i32) -> i32 {
    let m: usize = m as usize;
    let n: usize = n as usize;
    let mut cache = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            if i == 0 || j == 0 {
                cache[i][j] = 1;
            } else {
                cache[i][j] = cache[i-1][j] + cache[i][j-1];
            }
        }
    }
    return cache[m-1][n-1] as i32;
}

// https://leetcode-cn.com/problems/unique-paths-ii/solution/
pub fn unique_paths_with_obstacles2(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();    

    let mut cache = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            if obstacle_grid[i][j] == 1 {
                cache[i][j] = 0;
            } else if i == 0 && j == 0 {
                cache[i][j] = 1;
            } else if i == 0 {
                cache[i][j] = cache[i][j-1];
            } else if j == 0 {
                cache[i][j] = cache[i-1][j];
            } else {
                cache[i][j] = cache[i-1][j] + cache[i][j-1];
            }
        }
    }
    return cache[m-1][n-1];
}

// https://leetcode-cn.com/problems/house-robber/submissions/
pub fn rob(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    } else if len == 1 {
        return nums[0];
    } else if len == 2 {
        return nums[0].max(nums[1]);
    } // else len > 2
    let mut m1 = nums[0];
    let mut m2 = nums[1].max(m1);
    for i in 2..nums.len() {
        println!("m1 {} m2 {}", m1, m2);
        m1 = (m1 + nums[i]).max(m2);
        let temp = m2;
        m2 = m1;
        m1 = temp;
    }
    println!("m1 {} m2 {}", m1, m2);
    return m2;
}

// https://leetcode-cn.com/problems/maximum-product-subarray/submissions/
pub fn max_product(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0; }

    let (mut max, mut min) = (1, 1);
    let mut res = std::i32::MIN; 

    let len = nums.len();
    // 由于有 if 在循环里面，所以速度慢！ 
    for n in nums {
        let t_max = max;
        let t_min = min;
        max = (t_max * n).max(n).max(t_min * n);
        min = (t_min * n).min(n).min(t_max * n);
        res = res.max(max);
    }
    println!("{}", res);
    return res;
}

// https://leetcode-cn.com/problems/gu-piao-de-zui-da-li-run-lcof/
// 由于只买卖一次，所以只需要记录最低价格就好了

pub fn max_profit(mut prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut cost = 1<<30;
    for i in 0..prices.len() {
        cost = cost.min(prices[i]);
        profit = (prices[i] - cost).max(profit);
    }
    return profit;
}


// https://leetcode-cn.com/problems/word-break/
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    if word_dict.is_empty() { return false; }

    let len = s.len();
    let mut dp: Vec<bool> = vec![false; len+1];
    dp[0] = true;
    for i in 0..len {
        if !dp[i] { continue; }
        for w in &word_dict {
            let end = i + w.len();
            if end <= len && !dp[end] && &s[i..end] == w.as_str() {
                dp[end] = true;
            }
        }
    }
    dp[len] 
}

// https://leetcode-cn.com/problems/maximum-length-of-repeated-subarray/solution/
// 相当于填表
pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let row = a.len();
    let col = b.len();
    let mut dp = vec![vec![0; col]; row]; 
    let mut res = 0;

    for i in 0..row {
        for j in 0..col {
            if a[i] == b[j] {
                let last = if ( i == 0 || j == 0 ) { 0 } else { dp[i-1][j-1] };
                dp[i][j] = last + 1;
                res = res.max(dp[i][j]);
            } else {
                dp[i][j] = 0;
            }
        }
    }
    return res as i32;
}

// https://leetcode-cn.com/problems/unique-paths-ii/
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let row = obstacle_grid.len();
    let col = obstacle_grid[0].len();
    let mut dp = vec![vec![0; col]; row];

    // init first row and col
    for i in 0..row {
        for j in 0..col {
            if obstacle_grid[i][j] == 0 {
                if i == 0 && j == 0 {
                    dp[i][j] = 1;
                } else if i == 0 {
                    dp[i][j] = dp[i][j-1];
                } else if j == 0 {
                    dp[i][j] = dp[i-1][j];
                } else {
                    dp[i][j] = dp[i-1][j] + dp[i][j-1];
                }
            } else {
                // 遇到障碍了，但一开始我们就是初始化为0的，所以这里其实可以不写
                dp[i][j] = 0;
            }
        }
    }
    return dp[row-1][col-1];
}


// https://leetcode-cn.com/problems/re-space-lcci/
pub fn respace(dictionary: Vec<String>, sentence: String) -> i32 {
    42
}

// https://leetcode-cn.com/problems/li-wu-de-zui-da-jie-zhi-lcof/
pub fn max_value(mut grid: Vec<Vec<i32>>) -> i32 {
    let row = grid.len();
    let col = grid[0].len();
    for i in 0..row {
        for j in 0..col {
            if i == 0 && j == 0 {
                // pass
            } else if i == 0 {
                grid[i][j] += grid[i][j-1];
            } else if j == 0 {
                grid[i][j] += grid[i-1][j];
            } else {
                grid[i][j] += grid[i-1][j].max(grid[i][j-1]);
            }
        }
    }
    return grid[row-1][col-1];
}


// https://leetcode-cn.com/problems/triangle/solution/di-gui-ji-yi-hua-dp-bi-xu-miao-dong-by-sweetiee/
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let n = triangle.len();
    let mut dp = vec![0; n+1];

    for i in (0..n).rev() {
        for j in 0..=i {
            println!("i, j = {}, {}", i, j);
            dp[j] = dp[j].min(dp[j+1]) + triangle[i][j];
        }
    }
    return dp[0];
}

fn main()
{
    // generate_parenthesis(4);
//     println!("(1,1) {}", unique_paths_iter(1, 1));
//     println!("(2,2) {}", unique_paths_iter(2, 2));
//     println!("(3,2) {}", unique_paths_iter(3, 2));
//     println!("(2,3) {}", unique_paths_iter(2, 3));
    // rob([1, 3, 1, 3, 100].to_vec());
    // max_product([-2,0,-1].to_vec());
    // max_product([-1,-2,-9,-6].to_vec());
    // max_profit([1,2,3].to_vec());
    // word_break("leetcode".to_string(), ["leet".to_string(), "code".to_string()].to_vec());
    // dbg!(find_length([1,2,3,2,1].to_vec(), [3,2,1,4,7].to_vec()));
    // dbg!(max_profit_cool([1,2,3,0,2].to_vec()));
    let tri = [
        [2].to_vec(),
        [3,4].to_vec(),
        [6,5,7].to_vec(),
        [4,1,8,3].to_vec()
    ].to_vec();
    dbg!(minimum_total(tri));
}