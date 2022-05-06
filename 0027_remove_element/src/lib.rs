pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    println!("Vector without value {}: {:?}", val, nums);
    let len_nums = nums.len() as i32;

    len_nums
}