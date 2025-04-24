use std::collections::HashMap;
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        let mut l = 0;
        let mut r = 0;

        let mut count = 0;
        let distinct_values = Self::get_distinct_values(&nums);

        let mut map = HashMap::new();

        for l in 0..nums.len() {
            while count < distinct_values && r < nums.len() {
                let r_count = map.entry(nums[r]).or_insert(0);
                if *r_count == 0 {
                    count += 1;
                }
                *r_count += 1;
                r += 1;
            }

            if count == distinct_values {
                let l_count = map.entry(nums[l]).or_insert(0);
                *l_count -= 1;
                if *l_count == 0 {
                    count -= 1;
                }
                res += nums.len() - r + 1;
            }
        }
        return res as i32;
    }

    pub fn get_distinct_values(list: &Vec<i32>) -> i32 {
        let mut count = 0;
        let mut map = HashMap::new();
        for i in list {
            let my_bool = map.entry(i).or_insert(false);
            if !*my_bool {
                count += 1;
            }
            *my_bool = true;
        }
        return count;
    }
}