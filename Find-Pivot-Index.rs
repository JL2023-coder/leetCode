impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut bw = 0;
        let mut fw = 0;
        for i in 0..n {
            bw += nums[i]
        }

        for i in 0..n {
            fw += nums[i];
            if fw == bw {
                return i as i32;
            }
            bw -= nums[i];
        }

        return -1;
    }
}