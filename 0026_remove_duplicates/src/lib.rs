use itertools::Itertools;

pub fn remove_duplicates(nums: Vec<i32>) -> i32 {
    let unique_vec: Vec<_> = nums.into_iter().unique().collect();
    println!("Vector without duplicates: {:?}", unique_vec);
    let len_unique_vec = unique_vec.len() as i32;

    len_unique_vec
}