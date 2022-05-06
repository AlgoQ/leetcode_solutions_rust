pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i1, item1) in nums.iter().enumerate() {
        for (i2, item2) in nums[1..].iter().enumerate() {
            if item1 + item2 == target && i1 != i2 + 1 {
                return vec![i1 as i32, i2 as i32 + 1];
            }
        }
    }
    panic!("None of the numbers match up with the target");
}