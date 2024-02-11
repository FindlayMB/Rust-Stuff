
mod problems;
use problems::{
    valid_anagrams::run_valid_anagrams,
    two_sum::run_two_sum, 
    find_odd_int::run_find_odd, 
    top_k_freq_elem::run_top_k_frequent, 
    product_except_self::run_product_except_self 
    };


fn main() {
    // Comment out any functions that don't need to be tested
    // If they are commented out will give a unused code warning
    run_valid_anagrams();

    run_two_sum();

    run_find_odd();

    run_top_k_frequent();

    run_product_except_self();
    
}
