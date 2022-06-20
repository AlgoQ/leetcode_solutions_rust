pub fn longest_palindrome(s: String) -> String {
    let mut results: Vec<String> = vec![]; 
    let mut palindrome_vec: Vec<char> = vec![];

    for (i, c) in s.chars().enumerate() {
        palindrome_vec.push(c);
        
        for start_i in 0..i {
            let palindrome: String = palindrome_vec[start_i..].into_iter().collect();
            let rev_palindrome: String = palindrome.chars().rev().collect();
            if rev_palindrome == palindrome {
                results.push(palindrome.clone());
            }
        }
    }
    let longest_palindrome = results.iter().max_by_key(|s| s.len()).unwrap();

    longest_palindrome.to_string()
}