pub fn search_insert(mut nums: Vec<i32>, target: i32) -> i32 {
    let mut iter = nums.iter();
    let search_pos = iter.position(|x| x == &target);
    if search_pos.is_none() {
        nums.push(target);
        nums.sort();
        
        return  search_insert(nums, target);
    }
    else {
        return search_pos.unwrap() as i32;
    }
}