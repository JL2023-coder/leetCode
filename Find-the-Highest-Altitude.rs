impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut curr = 0;
        let mut max = 0;
        for i in gain {
            curr += i;
            if curr > max {
                max = curr;
            }
        }
        return max;
    }
}