
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut group_count: Vec<i32> = vec![0; 37];
        let mut max: i32 = 0;
        let mut count: i32 = 0;
        for i in 1..n + 1 {
            let digit_sum: usize = digit_sum(i);
            let c = group_count.get_mut(digit_sum).unwrap();
            *c += 1;

            if *c > max {
                max = *c;
                count = 1;
            }
            else if *c == max {
                count += 1;
            }    
        }

        return count;

        fn digit_sum(n: i32) -> usize {
            let mut sum: i32 = 0;
            let mut num = n;
            while num >= 1 {
                sum += num % 10;
                num = num / 10;
            }
            return sum as usize;
        }
    }
}
