impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return \1\.to_string();
        }

        let mut s: String = String::from(\1\);

        for i in 1..n {
            let mut newString: String = String::new();
            let mut count: usize = 1;
            let mut curr: char = 'q';
            let s_iter = s.chars();
            let mut i: usize = 0;
            
            for c in s.chars() {
                if i == 0 {
                    curr = c;
                }
                else if c == curr {
                    count += 1;
                }  
                else {
                    newString.push_str(&count.to_string());
                    newString.push(curr);
                    count = 1;
                    curr = c;
                }  

                if i == s.len() - 1 {
                    newString.push_str(&count.to_string());
                    newString.push(curr);
                }
                i += 1;       
            }
            s = newString;
        }

        return s;

    }
}