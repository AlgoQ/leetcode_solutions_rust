pub fn zigzag(s: String, num_rows: i32) -> String{
    let mut zigzag_vec: Vec<(i32, char)> = vec![];
    let mut row_index = 0;
    let mut gaining = true;
    let mut output = String::new();
    
    for c in s.chars() {
        zigzag_vec.push((row_index, c));
        
        if gaining && row_index < num_rows-1 {
            row_index += 1;
        }
        else if gaining && row_index == num_rows-1 {
            gaining = false;
            row_index -= 1;
        }
        else if !gaining && row_index > 0 {
            row_index -= 1;
        }
        else if !gaining && row_index == 0 {
            gaining = true;
            row_index += 1;
        }
    }

    for num in 0..num_rows {
        for t in zigzag_vec.iter() {
            println!("{:?}", t);
            let (i, c) = t;

            if *i == num {
                output.push(*c); 
            }
        }
    }
    output
}
