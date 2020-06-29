// https://leetcode-cn.com/problems/divide-two-integers/
// 越界条件：
// 由于负值比正值的绝对值要大1，因此，被除数为负值的最小值，除数为-1时，
// 得到的商将比最大的正数大1


// 辗转相减法，对于极端情况特慢
// 对正数：减到正数为负
// 一正一负: 加到正数为负
// 一负一正: 加到负数为正
// 两负数：减到负数为正
pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let mut res = -1;
    let mut dividend = dividend;
    let divisor = divisor;
    let mut sign = 1;

    if dividend == 0 || divisor == 0 { return 0; }
    if dividend == -2147483648 && divisor == -1 { return 2147483647; }
    if dividend == -2147483648 && divisor == 1 { return -2147483648; }
    

    if dividend > 0 && divisor > 0 {
        while dividend >= 0 {
            dividend = dividend - divisor;
            res += 1;
        }
    } else if dividend < 0 && divisor < 0 {
        while dividend <= 0 {
            dividend = dividend - divisor;
            res += 1
        }
    } else if dividend > 0 {
        sign = -sign;
        while dividend >= 0 {
            dividend = dividend + divisor;
            res += 1
        }
    } else {
        sign = -sign;
        while dividend <= 0 {
            dividend = dividend + divisor;
            res += 1
        }
    }

    return res * sign;
}

// 不要让 leetcode 模糊了你的算法
// 基本的思路是用位移先减去倍数，再处理一些溢出情况
pub fn divide2(dividend: i32, divisor: i32) -> i32 {
    // 加速辗转相除法
    // 先计算出 2^n * divisor 后的余数
    if divisor == 1 { 
        return dividend; 
    } else if divisor == -1 && dividend == -2147483648 {
        return 2147483647;
    } else if divisor == -1 {
        return -dividend;
    } else if dividend == 0 { 
        return 0;
    } else if dividend == divisor {
        return 1;
    } else if divisor == -2147483648 {
        return 0;
    }

    let mut dividend = dividend;
    let mut divisor = divisor;
    let mut times = 0;
    // 处理负号，异或运算之后，如果两数异号，最高位为1,整体为负，结束要乘-1
    let sign = if (dividend ^ divisor) < 0 { -1 } else { 1 };
    if dividend < 0 {
        // 直接取负可能会溢出，所以先减半。
        dividend = dividend >> 1;
        times += 1;
        dividend = -dividend;
    }
    if divisor < 0 {
        divisor = -divisor;
    }

    let mut dividend2 = dividend;
    dbg!(dividend2, divisor);

    // 以下算法仅适用于两数为正整数，因此，需要在这之前完成转换
    while dividend2 >= divisor {
        dividend2 = dividend2 >> 1;
        times += 1;
    }
    if times == 0 { return 0; } // 小于除数
    let mut fold = 1 << (times - 1); // 计算出倍数
    dbg!(times, fold, dividend2);
    if dividend2 == 1 {  // 刚好除完，这种情况如果相减反正会溢出，所以先处理
        return fold * sign; 
    } 
    let mut dividend2 = dividend - (divisor<<(times -1));
    dbg!(dividend2);
    while dividend2 > divisor {
        dividend2 = dividend2 - divisor;
        fold += 1;
    }
    return fold * sign;
}



fn main() {
    // let x = divide(-2147483648, 1);
    // println!("{}", x);
    // println!("{}", 30<<1);
    println!("{}",divide2(-2147483648, 2));
    println!("{}",divide2(1, 2));
    println!("{}",divide2(10, 3));
    println!("{}", divide2(2147483647,2));
}