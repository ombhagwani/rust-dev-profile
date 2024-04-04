fn shortest_word(sentence: &str) -> Option<&str> {
    sentence.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let sentence = "hey there this is ombhagwani";
    
    if let Some(shortest) = shortest_word(sentence) {
        println!("The shortest word is: {}", shortest);
    } else {
        println!("No words found in the sentence.");
    }
}
