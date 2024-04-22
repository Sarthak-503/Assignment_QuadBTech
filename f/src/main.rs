fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = strings[0];
    let mut prefix = String::new();

    'outer: for (i, char) in first_string.chars().enumerate() {
        for string in strings.iter().skip(1) {
            if let Some(c) = string.chars().nth(i) {
                if c != char {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(char);
    }

    prefix
}

fn main() {
    let strings = ["flower", "flow", "flight"];
    let prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", prefix);

    let strings2 = ["dog", "racecar", "car"];
    let prefix2 = longest_common_prefix(&strings2);
    println!("Longest common prefix: {}", prefix2);
}
