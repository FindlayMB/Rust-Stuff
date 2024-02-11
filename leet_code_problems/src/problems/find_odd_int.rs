use std::collections::{HashMap, HashSet};
pub fn find_odd(arr: &[i32]) -> i32 {
    
    let key_set: HashSet<i32> = HashSet::from_iter(arr.iter().cloned());
    
    let mut count_map:HashMap<i32,i32> = HashMap::new();

    for key in key_set {
        count_map.insert(key, arr.iter().filter(|&x| *x==key).count() as i32);
        if count_map[&key] % 2 == 1 {
            return key;
        }
    }


    // Weird but valid solution
    let set: Option<HashSet<i32, >> = Some(HashSet::from_iter(arr.iter().cloned()));
    for key in set.unwrap() {
        if (arr.iter().filter(|&x| *x==key).count() as i32) %2 == 1{
            return key;
        }
    }
    
    return 0;
}

pub fn run_find_odd() {
    let x: i32 = find_odd(&vec![20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5]);
    
    println!("\nFind Odd Int");
    println!("Expected: 5");
    println!("Actual:   {x}");
}