use std::cmp::{min, max};
// https://leetcode-cn.com/problems/container-with-most-water
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut li = 0;
    let mut ri = height.len() - 1;
    let mut area = 0;
    let mut area_tmp = 0;
    let mut h: i32 = 0;

    while (li < ri) {
        h = (ri - li) as i32;
        area_tmp = h * min(height[li], height[ri]); 
        area = max(area_tmp, area);
        if height[li] < height[ri] { li += 1; } else { ri -= 1; }
    }
    return area;
}


