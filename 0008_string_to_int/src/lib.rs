pub fn string_to_int(s: String) -> i32 {
    let allowed_chars: [char; 12] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-'];
    let mut output_str =  String::new();

    for c in s.chars(){
        if allowed_chars.contains(&c) {
            output_str.push(c);
        }
    }
    
    output_str.parse::<i32>().unwrap() 
}
