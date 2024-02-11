use std::collections::{HashMap, HashSet};

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() < t.len() {
        return false;
    }
    let set1: HashSet<char> = s.chars().collect();

    let mut t_m: HashMap<char, usize> = HashMap::new();

    for pair in set1 {
        t_m.insert(pair, t.chars().filter(|x| *x == pair).count());
    }

    for (c, n) in &t_m {
        let b: bool = *n == s.chars().filter(|x| x == c).count();
        if !b {
            return false;
        }
    }

    return true;
}

pub fn run_valid_anagrams() {
    let s:String = String::from("anagram");
    let t:String = String::from("nagaram");

    let result:bool = is_anagram(s.clone(), t.clone()); 

    println!("\nValid Anagram");
    println!("Is {t} a valid anagram of {s}: {}",result);
}