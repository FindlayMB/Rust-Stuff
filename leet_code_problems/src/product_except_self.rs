
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {    
    let zero_amt = nums.iter().filter(|&x| *x==0).count();
    
    if zero_amt > 1 {
        return vec![0; nums.len()];
    } 
    
    let mut res: Vec<i32> = vec![1; nums.len()];

    for i in 1..nums.len(){
        res[i] = res[i-1] * nums[i-1];
    }

    let mut temp = 1;

    for i in (0..nums.len()).rev(){
        res[i] *= temp;
        temp *= nums[i];
    }
    

    return res;
}

