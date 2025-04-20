use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::<char, i32>::new();

        for c in s.chars() {
            let mut count = map.entry(c).or_insert(0);
            *count += 1;
        }

        for c in t.chars() {
            let mut check = map.entry(c).or_insert(0);
            if *check == 0{
                return false;
            }
            else {
                *check -= 1;
            }
        }

        return true;
    }
}