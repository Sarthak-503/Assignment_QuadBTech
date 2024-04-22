fn reverse_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    chars.into_iter().collect()
}

fn main() {
    let input = "Hello, world!";
    let reversed = reverse_string(input);
    println!("Reversed string: {}", reversed);
}
