pub fn reverse(x:i32) -> i32 {
    let x_str = x.to_string();
    let mut negative = false;
    let mut output_str = String::new();
    
    for c in x_str.chars().rev() {
        if c == '-' {
            negative = true;
            continue
        }
        output_str.push(c);
    }
    
    let mut output = output_str.parse::<i32>().unwrap(); 
    if negative {
       output = output * -1; 
    }
    output
}
