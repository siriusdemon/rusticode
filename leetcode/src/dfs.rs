// dfs 系列

// https://leetcode-cn.com/problems/ba-shu-zi-fan-yi-cheng-zi-fu-chuan-lcof/comments/
// 一个数学，可以有多少种翻译？
// a-z -> 0-25
// 跟小青蛙跳台阶不同，这里的 2 位读法不一定取得到。
pub fn translate_num(num: i32) -> i32 {
    if num <= 9 || (num >= 26 && num < 100) { return 1; }
    let last2 = num % 100;
    if 9 < last2 && last2 < 26 {
        return translate_num(num / 10) + translate_num(num / 100);
    } else {
        return translate_num(num / 10);
    }
}