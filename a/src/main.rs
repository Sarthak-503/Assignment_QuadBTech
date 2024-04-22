fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn main() {
    let test_str = "A man, a plan, a canal, Panama";
    if is_palindrome(test_str) {
        println!("{} is a palindrome.", test_str);
    } else {
        println!("{} is not a palindrome.", test_str);
    }
}
