use std::collections::HashSet;

pub fn pigify(string: &str) -> String {
    let vowels : HashSet<&str> = vec!["A", "E", "I", "O", "U", "Y", "a", "e", "i", "o", "u", "y"].into_iter().collect();
    //consonant goes to end and gets appended with -ay "irst-fay"
    //vowel gets appended with hay "apple-hay"
    //vowels are A, E, I, O, U, and sometimes Y

    let mut string_iter = string.chars();

    let first_char: String = string_iter.next().unwrap().to_string();

    if vowels.contains(&first_char.as_str()) {
        return String::from(string) + "-hay";
    }

    return String::from(string_iter.as_str()) + "-" + &first_char + "ay";
}