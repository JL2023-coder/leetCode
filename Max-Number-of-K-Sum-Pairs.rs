use std::collections::HashMap;
use std::cmp::min;
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut max: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::<i32,i32>::new();

        for v in nums {
            let mut count = map.entry(v).or_insert(0);
            *count += 1;
        }

        for (&key, &value) in &map {
            let to_find = k - key;
            let value_2 = map.get(&to_find).copied().unwrap_or(0);
            max += min(value, value_2);
        }
        return max / 2;
    }
}