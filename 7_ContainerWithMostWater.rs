use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut res = 0;


        while l < r {
            res = cmp::max(res, cmp::min(height[l], height[r]) * (r - l) as i32);

            if height[l] > height[r] {
                r -= 1;
            }
            else {
                l += 1;
            }
        }

        res
        }
}
