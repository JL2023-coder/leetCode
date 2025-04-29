use std::collections::LinkedList;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut n = nums.len();
        let mut max = i32::MIN;
        let mut res: i64 = 0;
        for i in 0..n {
            if nums[i] > max {
                max = nums[i]
            }
        }

        let mut list: LinkedList<u32> = LinkedList::new();
        for i in 0..n {
            if nums[i] == max {
                list.push_back(i as u32);
            }
            if list.len() as i32 > k {
                list.pop_front();
            }
            if list.len() as i32 == k {
                res += *list.front().unwrap() as i64 + 1;
            }
        }
        return res;
    }
}