// https://leetcode-cn.com/problems/minimum-size-subarray-sum/solution/jin-tian-mei-tu-yi-dong-chuang-kou-xian-zhao-dao-y/
pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
    let mut minlen = 9999_999;
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;

    while j < nums.len() {
        sum += nums[j];
        while sum >= s {
            // 更新minlen之后，开始优化
            minlen = minlen.min(j-i+1);
            // 尝试删除最远的一个
            sum -= nums[i];
            // 指针前移
            i += 1;
            // 当 s 不满足时要退出，此时 minlen 持有最小的长度
            // 重复此过程直到遍历完成
        }
        j += 1;
    }
    if minlen == 9999_999 { return 0; } 
    return minlen;
}