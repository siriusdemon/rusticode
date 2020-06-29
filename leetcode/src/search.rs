// https://leetcode-cn.com/problems/search-insert-position/
// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
pub fn search_insert(nums: &Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    if len == 0 { return 0; }

    let mut left = 0;
    let mut right = len;
    while right > left {
        let mid = (right + left) / 2;
        if nums[mid] == target { return mid as i32; }
        else if target < nums[mid] {
            // println!("right {} -> {}", right, mid);
            right = mid;
        } else {
            // println!("left {} -> {}", left, mid+1);
            left = mid + 1;
        }
        // println!("left {}- right {}", left, right);
    }

    return left as i32; 
}

// https://leetcode-cn.com/problems/zai-pai-xu-shu-zu-zhong-cha-zhao-shu-zi-lcof/comments/
// 统计一个数字在排序数组中出现的次数。
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 { return 0; }
    let mut left = 0;
    let mut right = nums.len() - 1;

    while right > left {
        let mid = (right + left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    // 最终落在 left 上
    let mut count = 0;
    while left < nums.len() && nums[left] == target {
        count += 1;
        left += 1;
    }
    return count;
}

fn main()
{
    let nums = [1, 2, 3, 5, 6, 9].to_vec();
    dbg!(search_insert(&nums, 4));
    dbg!(search_insert(&nums, 10));
    dbg!(search([5,7,7,8,8,10].to_vec(), 8));
    dbg!(search([].to_vec(), 0));
    dbg!(search([2,2].to_vec(), 1));

}