fn are_characters_unique(s: &str) -> bool {
    let mut characters = vec![];

    for c in s.chars() {
        let lower_c = c.to_lowercase().next().unwrap();
        if characters.contains(&lower_c) {
            return false;
        }
        characters.push(lower_c);
    }

    true
}

fn main() {
    let test_cases = vec!["abcd", "abCdefAaf", "aabcd"];

    for s in test_cases {
        println!("{} -> {}", s, are_characters_unique(s));
    }
}