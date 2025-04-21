use std::cmp::{min, max};
impl Solution {
    pub fn number_of_arrays(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut my_new_array: Vec<i64> = Vec::<i64>::new();
        let mut min_value: i64 = 0;
        let mut max_value: i64 = 0;
        my_new_array.push(0);
        for i in 0..nums.len() {
            let my_value = my_new_array[i] + nums[i] as i64;
            min_value = min(my_value, min_value);
            max_value = max(my_value, max_value);
            my_new_array.push(my_value);
        }

        max_value += lower as i64 - min_value;

        let differences: i32 = max(0, upper as i64 - max_value + 1) as i32;
        return differences;
    }
}