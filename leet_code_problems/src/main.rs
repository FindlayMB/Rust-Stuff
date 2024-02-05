mod valid_anagrams;
mod two_sum;
mod find_odd_int;
mod top_k_freq_elem;
mod product_except_self;
// #[test]
pub fn run_valid_anagrams() {
    let s:String = String::from("anagram");
    let t:String = String::from("nagaram");

    let result:bool = valid_anagrams::is_anagram(s.clone(), t.clone()); 

    println!("Is {t} a valid anagram of {s}: {}",result);
}

// #[test]
pub fn run_two_sum() {
    let nums:Vec<i32> = vec![2,7,11,15];
    let target:i32 = 9;

    let result:Vec<i32> = two_sum::two_sum(nums,target);

    println!("Expected: [0, 1]");
    println!("Actual:   [{}, {}]", result[0], result[1]);
}

// #[test]
pub fn run_find_odd() {
    let x: i32 = find_odd_int::find_odd(&vec![20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5]);
    
    println!("{x}");
}

// #[test]
pub fn run_top_k_frequent() {
    let nums: Vec<i32> = vec![1,1,1,1,2,2,2,3];
    let k:i32 = 2;

    let expected:Vec<i32> = vec![1,2];
    let result:Vec<i32> = top_k_freq_elem::top_k_frequent(nums.clone(), k);

    println!("Expected: {:?}",expected);
    println!("Actual:   {:?}",result);
    assert_eq!(expected,result);
}

#[test]
pub fn run_product_except_self() {
    let nums: Vec<i32> = vec![1,2,3,4];
    
    let expected:Vec<i32> = vec![24,12,8,6];

    let result:Vec<i32> = product_except_self::product_except_self(nums.clone());

    println!("Expected: {:?}",expected);
    println!("Actual:   {:?}",result);
    assert_eq!(expected,result);
}

fn main() {

    // run_top_k_frequent();


    
}
