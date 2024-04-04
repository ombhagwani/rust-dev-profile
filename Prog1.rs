fn palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn main() {
    let input = "racecar";
    if palindrome(input) {
        println!("{} is a palindrome.", input);
    } else {
        println!("{} is not a palindrome.", input);
    }
}
