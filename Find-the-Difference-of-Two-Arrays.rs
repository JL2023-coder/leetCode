use std::collections::HashMap;
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();

        let mut ans1 = Vec::new();
        let mut ans2 = Vec::new();

        for i in nums1 {
            map1.insert(i, 1);
        }

        for i in nums2 {
            map2.insert(i, 1);
        }

        for k in map1.keys() {
            if *map2.entry(*k).or_insert(0) == 0 {
                ans1.push(*k);
            }
        }

        for k in map2.keys() {
            if *map1.entry(*k).or_insert(0) == 0 {
                ans2.push(*k);
            }
        }

        return vec!(ans1, ans2);
    }
}