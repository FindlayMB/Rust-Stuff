use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut seen:HashMap<i32,i32> = HashMap::new();

    for (i, val) in nums.iter().enumerate() {
        let x: i32 = target - val;
        if let Some(&j) = seen.get(&x){
            return vec![j as i32, i as i32];
        }
        seen.insert(*val, i as i32);
    }

    unreachable!();
}