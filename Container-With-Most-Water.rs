use std::cmp::min;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;

        let mut max: i32 = 0;

        loop {
            let curr = (r - l) as i32 * min(height[l], height[r]);
            if curr > max {
                max = curr;
            }
            
            if height[l] > height[r] {
                r -= 1;
            }
            else {
                l += 1;
            }

            if l == r {
                return max;
            }
        }
    }
}