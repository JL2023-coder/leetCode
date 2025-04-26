use std::cmp::min;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut mini: i32 = -1;
        let mut maxi: i32 = -1;
        let mut start: i32 = -1;

        let mut res = 0;

        for i in 0..nums.len() {
            if nums[i] >= min_k && nums[i] <= max_k {
                if nums[i] == min_k {
                    mini = i as i32;
                }
                if nums[i] == max_k {
                    maxi = i as i32;
                }
                if mini != -1 && maxi != -1 && min(mini, maxi) > start{
                    res += (min(mini, maxi) - start) as i64;
                }
            }
            else {
                start = i as i32;
            }   
        }

        return res;
    }
}
