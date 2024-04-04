fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = strings[0].clone();
    for s in &strings[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    prefix
}

fn main() {
    let words = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    let lcp = longest_common_prefix(&words);
    println!("Longest Common Prefix: {}", lcp);
}
