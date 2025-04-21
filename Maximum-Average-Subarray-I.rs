impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut curr: f64 = 0.0;
        let mut max: f64 = 0.0;
        let mut i: usize = 0;

        loop {
            if i == k as usize{
                max = curr;
                break;
            }
            curr += nums[i] as f64;
            i += 1;
        }

        loop {
            if i == nums.len() {
                return max / k as f64;
            }
            else {
                curr += nums[i] as f64;
                curr -= nums[i - k as usize] as f64; 
                if curr > max {
                    max = curr;
                }
            }
            i += 1;
        }
    }
}