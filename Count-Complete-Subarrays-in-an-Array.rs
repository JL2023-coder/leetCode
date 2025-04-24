use std::collections::HashMap;
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut map_values = HashMap::<i32, usize>::new();
        let mut distinct_values = 0;
        for i in &nums {
            let c = map_values.entry(*i).or_insert(0);
            if *c == 0 {
                distinct_values += 1;
            }
            *c += 1;
        }

        for i in 0..nums.len() { 
            let mut counter = 0;
            let mut encountered_values = HashMap::<i32, usize>::new();
            for j in i..nums.len() {
                let c = encountered_values.entry(nums[j]).or_insert(0);
                if *c == 0 {
                    counter += 1;
                }
                *c += 1;

                if counter >= distinct_values {
                    res += 1;
                }
            }
        }

        return res;
    }
}