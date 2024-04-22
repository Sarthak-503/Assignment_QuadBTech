fn find_shortest_word(input: &str) -> Option<&str> {
    input.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let input = "The quick brown fox jumps over the lazy dog";
    if let Some(shortest_word) = find_shortest_word(input) {
        println!("The shortest word is: {}", shortest_word);
    } else {
        println!("No words found in the input string.");
    }
}
