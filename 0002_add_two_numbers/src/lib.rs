pub fn add_two_numbers(l1: Vec<u32>, l2: Vec<u32>) -> Vec<u32> {
    fn vec_to_u32(v: Vec<u32>) -> u32 {
        let mut my_string = "".to_owned();
        for i in v.iter().rev(){
            my_string.push_str(&i.to_string());
        }
        my_string.parse::<u32>().unwrap() 
    }
    
    fn u32_to_rev_vec(num: u32) -> Vec<u32> {
        const RADIX: u32 = 10;
        let mut v = vec![];
        let s: String = num.to_string();
        for c in s.chars().rev() { 
            v.push(c.to_digit(RADIX).unwrap()); 
        }
        v
    }
    
    let sum = vec_to_u32(l1) + vec_to_u32(l2);
    u32_to_rev_vec(sum) 
}
