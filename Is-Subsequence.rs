impl Solution {
    pub fn is_subsequence(sub: String, main: String) -> bool {
        let sub_list:Vec<char> = sub.chars().collect();
        let length_sub_list: usize = sub_list.len();

        if length_sub_list == 0 {
            return true;
        }

        let mut index_sub_list: usize = 0;
        for c in main.chars() {
            if c == sub_list[index_sub_list] {
                index_sub_list += 1;
                if index_sub_list == length_sub_list {
                    return true;
                }
            }
        }
        
        return false;
    }
}