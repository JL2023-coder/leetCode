impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut kays = k;
        let mut curr = 0;
        let mut max = 0;
        let mut r = 0;


        for l in 0..nums.len() {
            while r < nums.len() && (kays > 0 || nums[r] == 1){
                if nums[r] == 0 {
                    kays -= 1;
                }
                curr += 1;
                if curr > max {
                    max = curr;
                }
                r += 1;
            }
            if nums[l] == 0 {
                kays += 1;
            }
            curr -= 1;
        }

        return max;
    }
}