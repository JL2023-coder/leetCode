impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for r in 2..nums.len() {
            if (nums[r] + nums[r-2]) as f32 == nums[r-1] as f32 / 2.0 {
                res += 1;
            }
        }
        return res;
    }
}