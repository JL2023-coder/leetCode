use std::collections::HashMap;
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut group_count: HashMap<i32, i32> = HashMap::<i32, i32>::new();
        let mut max: i32 = 0;
        let mut count: i32 = 0;
        for i in 1..n + 1 {
            let digit_sum = digit_sum(i);
            let c = group_count.entry(digit_sum).or_insert(0);
            *c += 1;
        }

        for c in group_count.values() {
            if *c > max {
                max = *c;
                count = 1;
            }
            else if *c == max {
                count += 1;
            }
        }
        return count;

        fn digit_sum(n: i32) -> i32 {
            let mut sum: i32 = 0;
            let mut num = n;
            while num >= 1 {
                sum += num % 10;
                num = num / 10;
            }
            return sum;
        }
    }
}
