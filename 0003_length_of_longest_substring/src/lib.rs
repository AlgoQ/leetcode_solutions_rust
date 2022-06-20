pub fn length_of_longest_substring(s: String) -> i32 {
    let mut substrings_len = vec![];
    let mut substring_vec = vec![];

    for c in s.chars() {
        if substring_vec.contains(&c){
            substrings_len.push(substring_vec.len() as i32);
            substring_vec = vec![c];
        }
        else {
            substring_vec.push(c);
        }
    }
    substrings_len.push(substring_vec.len() as i32);
    *substrings_len.iter().max().unwrap()
}

pub fn main(){
    println!("hello world");
    let s = "pwwkew".to_owned();
    let output = length_of_longest_substring(s);

    println!("{}", output);
}
