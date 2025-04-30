impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        for i in 0..n {
            if Self::is_even_num_of_digit(nums[i]) {
                res += 1;
            }
        }
        return res;
    }

    pub fn is_even_num_of_digit(mut num: i32) -> bool {
        let mut count = 0;
        while num > 0 {
            count += 1;
            num  = num / 10;
        }
        return count % 2 == 0;
    }
}