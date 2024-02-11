
use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts = HashMap::<i32, i32>::new();
    for num in nums {
        *counts.entry(num).or_default() += 1;
    }
    let mut freq_pairs: Vec<(i32, i32)> = counts.into_iter().collect();
    freq_pairs.sort_by_key(|x| -x.1);

    freq_pairs.into_iter().take(k as usize).map(|(num, _)| num).collect()
}

pub fn run_top_k_frequent() {
    let nums: Vec<i32> = vec![1,1,1,1,2,2,2,3];
    let k:i32 = 2;

    let expected:Vec<i32> = vec![1,2];
    let result:Vec<i32> = top_k_frequent(nums.clone(), k);

    println!("\nTop k Frequent");
    println!("Expected: {:?}",expected);
    println!("Actual:   {:?}",result);
    assert_eq!(expected,result);
}

