// https://leetcode-cn.com/problems/edit-distance/
pub fn min_distance(word1: String, word2: String) -> i32 {
    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();

    fn dp(i: i32, j: i32, c1: &Vec<char>, c2: &Vec<char>) -> i32 {
        if i < 0 {
            return j + 1;
        } else if j < 0 {
            return i + 1;
        } else if c1[i as usize] == c2[j as usize] {
            return dp(i-1, j-1, c1, c2);
        } else {
            return (dp(i-1, j, c1, c2) + 1) // insert
                      .min(dp(i, j-1, c1, c2) + 1) // delete
                      .min(dp(i-1, j-1, c1, c2) + 1); // substitude
        } 
    };

    let len1 = chars1.len() as i32;
    let len2 = chars2.len() as i32;
    
    if len1 == 0 || len2 == 0 {
        return len1 + len2;
    } else {
        return dp(len1-1, len2-1, &chars1, &chars2);
    }
}

pub fn min_distance_iter(word1: String, word2: String) -> i32 {
    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();

    let (len1, len2) = (chars1.len(), chars2.len());
    if len1 == 0 || len2 == 0 {
        return (len1 + len2) as i32;
    } 
    // else goes here 
    let mut dp: Vec<Vec<i32>> = vec![vec![0; len2+1]; len1+1];
    // init the first column and row
    for i in 0..=len1 { dp[i][0] = i as i32; }
    for j in 0..=len2 { dp[0][j] = j as i32; }
    // compute forward
    for i in 1..=len1 {
        for j in 1..=len2 {
            dp[i][j] = if chars1[i-1] == chars2[j-1] {
                dp[i-1][j-1]
            } else {
                1 + dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1])
            }
        }
    }
    return dp[len1][len2];
}