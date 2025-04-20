use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut colour_count: HashMap<i32, i32> = HashMap::<i32, i32>::new();

        Self::initalize_map(answers, &mut colour_count);

        let mut rabbits: i32 = 0;
        for (key, value) in colour_count {
            rabbits += Self::get_count(key as f32, value as f32);
        }

        return rabbits;
    }

    pub fn get_count(k: f32, v: f32) -> i32{
        if k == 0.0 {
            return v as i32;
        }
        let min: f32 = k + 1.0;
        let x: f32 = v / min;
        let x_ru: f32 = x.ceil();

        return (min * x_ru) as i32;
    }

    pub fn initalize_map(list: Vec<i32>, map: &mut HashMap<i32, i32>) {
        for c in list {
            let mut count = map.entry(c).or_insert(0);
            *count += 1;
        }
    }
}