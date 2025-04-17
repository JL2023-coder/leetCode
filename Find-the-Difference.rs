use std::collections::HashMap;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut map: HashMap<char, i32> = HashMap::<char, i32>::new();

        for c in s.chars() {
            let val = map.entry(c).or_insert(0);
            *val += 1;
        }
        

        for c in t.chars() {
            if *map.entry(c).or_insert(0) == 0 {
                return c;
            }
            map.entry(c).and_modify(|value| *value -= 1);
        }

        return '-';
    }
}