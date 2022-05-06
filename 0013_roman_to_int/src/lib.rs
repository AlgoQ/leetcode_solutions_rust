// Info - Wanted to make use of the C-like ENUM
enum RomanNum {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000
}

pub fn roman_to_int(s: String) -> i32 {
    let mut output: i32 = 0;
    let mut num_prev: i32 = 0;
    for c in s.chars().rev() { 
        let num = match c {
            'I' => RomanNum::I,
            'V' => RomanNum::V,
            'X' => RomanNum::X,
            'L' => RomanNum::L,
            'C' => RomanNum::C,
            'D' => RomanNum::D,
            'M' => RomanNum::M,
            _ => panic!("Invalid roman number")
        };
        
        let num = num as i32;

        if num_prev != 0 {
            if num < num_prev {
                output -= num;
            }
            else {
                output += num;
            }
        }
        else {
            output += num;
        }
        num_prev = num;
    }
    output
}