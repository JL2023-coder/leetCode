use std::collections::HashMap;
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut count = HashMap::new();
        let mut occ = HashMap::new();

        for n in arr {
            let c = count.entry(n).or_insert(0);
            *c += 1;
        }
        for v in count.values() {
            let c = occ.entry(v).or_insert(0);
            if *c == 1 {
                return false;
            }
            *c += 1;
        }
        return true;
    }
}