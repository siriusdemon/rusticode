// QuickSort by Mory
// 直观解释快排

// 修改为左闭右开
fn quicksort(array: &mut[i32], first: usize, last: usize) {
    if (last - first) > 1 {
        let midpoint = partition(array, first, last);
        quicksort(array, first, midpoint); 
        quicksort(array, midpoint + 1, last);
    }
}

fn  partition(array:  &mut[i32], first: usize, last: usize) -> usize {
    let last = last - 1;  // 左闭右开
    let pivot = array[last]; 

    let mut i: usize = first;
    let mut j: usize = last - 1;

    while i != j {
        if array[i] > pivot {
            swap(array, i, j);
            j = j - 1;
        } else {
            i = i + 1;
        }
    }

    if array[i] > pivot {
        swap(array, i, last);
    } else {
        i = i + 1;
        swap(array, i, last);
    }
    return i;
}


fn swap(array: &mut[i32], a: usize, b: usize) {
    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
}

// 由于快排每次都可以排好一位数字，所以可以用这个特点来解第k个数的问题：
// 给定一个无序数组，返回排序后第k个数，时间复杂度为log2n
fn quickkth(array: &mut [i32], first: usize, last: usize, k: usize) -> i32
{
    if (last - first) > 1 {
        let mid = partition(array, first, last);
        if mid == k - 1 { // 第k位的下标是k-1
            return array[mid];
        } else if mid < k - 1 {
            return quickkth(array, mid + 1, last, k);
        } else {
            return quickkth(array, first, mid, k);
        }
    } else {
        return array[first];
    }
}

// 求中位数
fn medium(array: &mut [i32], first: usize, last: usize) -> f32 {
    let len = last - first + 1;
    if len % 2 == 0 {
        let mid1 = len / 2;
        let mid2 = mid1 - 1;
        println!("mid1: {} mid2: {}", mid1, mid2);
        let mid1_val = quickkth(array, first, last, mid1);
        let mid2_val = quickkth(array, first, last, mid2);
        return (mid1_val + mid2_val) as f32 / 2.0; 
    } else {
        let mid = len / 2;
        let mid_val = quickkth(array, first, last, mid);
        return mid_val as f32;
    }
}

// https://leetcode-cn.com/problems/median-of-two-sorted-arrays/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by-w-2/
// 寻找两个有序列表中合并后的第k个数
// 注意，为了能够表示数据为空，在这里的首尾是左闭右开的 [)
fn find_kth_in_two_ordered_array(array1: &[i32], p1: usize, q1: usize, array2: &[i32], p2: usize, q2: usize, k: usize) -> i32
{
    let len1 = q1 - p1;
    let len2 = q2 - p2;
    
    if len1 == 0 {
        return array2[p2 + k - 1];
    } else if len2 == 0 {
        return array1[p1 + k - 1];
    }

    if k == 1 {
        return array1[p1].min(array2[p2]);
    }

    let m1 = p1 + len1.min(k / 2) - 1;
    let m2 = p2 + len2.min(k / 2) - 1;

    if array1[m1] < array2[m2] {
        return find_kth_in_two_ordered_array(array1, m1+1, q1, array2, p2, q2, k - (m1 - p1 + 1));
    } else {
        return find_kth_in_two_ordered_array(array1, p1, q1, array2, m2+1, q2, k - (m2 - p2 + 1));
    }

}

// 进阶，寻找两个有序列表的中位数
// https://leetcode-cn.com/problems/median-of-two-sorted-arrays/submissions/
fn find_medium_in_two_ordered_array(array1: &[i32], len1: usize, array2: &[i32], len2: usize) -> f32 {
    let len = len1 + len2;
    if len % 2 == 0 {
        let m1 = len / 2;
        let m2 = m1 + 1;
        let t1 = find_kth_in_two_ordered_array(array1, 0, len1, array2, 0, len2, m1);
        let t2 = find_kth_in_two_ordered_array(array1, 0, len1, array2, 0, len2, m2);
        return (t1 + t2)  as f32 / 2.0;
    } else {
        let m = len / 2 + 1;
        return find_kth_in_two_ordered_array(array1, 0, len1, array2, 0, len2, m) as f32;
    }
}


fn main() {
    let mut array = [31, -5, 11, 4, 2, 0, 12, -2, 13, 1, 21];
    let mut array = [0, 1];
    let len = array.len();
    println!("{:?}", array);
    // let mid = medium(&mut array, 0, 10);
    // println!("medium {}", mid);
    let k = 9;
    let val = quickkth(&mut array, 0, len, k);
    println!("the {}th val = {}", k, val);
    quicksort(&mut array, 0, len);
    println!("{:?}", array);
    // let a1 = [1, 20, 31, 43, 51];
    // let a2 = [2, 5, 21, 26, 37, 42];
    // let kth = find_kth_in_two_ordered_array(&a1, 0, 5, &a2, 0, 6, 11);
    // println!("find kth in two ordered array: {}", kth);
    // let m = find_medium_in_two_ordered_array(&a1, 5, &a2, 6);
    // println!("find medium in two ordered array: {}", m);
}