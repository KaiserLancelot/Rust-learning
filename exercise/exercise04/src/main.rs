fn main() {
    println!("{}", pig_latin("first"));
    println!("{}", pig_latin("apple"));
}

fn pig_latin(str: &str) -> String {
    let first_char = str.to_lowercase().chars().next().unwrap();

    if !vowel(first_char) {
        format!("{}-{}ay", &str[1..], first_char)
    } else {
        format!("{}-hay", str)
    }
}

fn vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
