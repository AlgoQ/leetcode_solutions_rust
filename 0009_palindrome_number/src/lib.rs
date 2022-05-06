pub fn is_palindrome(x: i32) -> bool {
    let x_str: String = format!("{:?}", x);
    let reversed_str = x_str.chars().rev().collect::<String>();
    // let reversed: i32 = reversed_str.parse().unwrap(); // Reverse back to i32

    x_str == reversed_str
}
