use std::cmp::min;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut good_arrs = Vec::new();
        let mut res = 0;
        let n = nums.len();

        let mut arr = Vec::new();
        for i in 0..n {
            if nums[i] >= min_k && nums[i] <= max_k {
                arr.push(nums[i]);
            }
            else {
                if arr.len() != 0 {
                    good_arrs.push(arr);
                }
                arr = Vec::new();
            }
        }
        if arr.len() != 0 {
            good_arrs.push(arr);
        }

        for i in good_arrs {
            let mut mini: i32 = -1;
            let mut maxi: i32 = -1;
            let mut start: i32 = -1;
            for r in 0..i.len() {
                if i[r] == min_k {mini = r as i32}
                if i[r] == max_k {maxi = r as i32} 
                res += (min(mini, maxi) - start) as i64;
            }

        }

        return res;
    }
}
