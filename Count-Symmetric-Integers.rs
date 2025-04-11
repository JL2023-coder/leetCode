impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    let mut count: i32 = 0;

    for n in low..high + 1 {
        let num_as_string:String = n.to_string();
        let len: i32 = num_as_string.len() as i32;
        let half: i32 = len / 2;
        if len % 2 != 0 {
            continue;
        }
        let mut half_counter: i32 = 0;
        let mut sum1: i32 = 0;
        let mut sum2: i32 = 0;
        for c in num_as_string.chars() {
            half_counter += 1;
            if let Some(digit) = c.to_digit(10) {
                if half_counter <= half {
                    sum1 += digit as i32;
                }
                else{
                    sum2 += digit as i32;
                }
            }   
        }
        if sum1 == sum2 {
            count += 1;
        }
    }

    return count;
    }
}