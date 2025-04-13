
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) { 
        let mut nonZero: usize = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                let temp: i32 = nums[i];
                nums[i] = nums[nonZero];
                nums[nonZero] = temp;
                nonZero += 1;
            }
        }
    }
}