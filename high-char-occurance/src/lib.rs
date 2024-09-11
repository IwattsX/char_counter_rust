use std::collections::HashMap;

/// Converts a string into a `HashMap` that counts the occurrences of each character.
///
/// # Arguments
///
/// * `s` - A string slice containing the characters to count.
///
/// # Returns
///
/// A `HashMap<char, i32>` where each key is a character, and each value is the count of that character in the string.
pub fn str_to_mapped_char(s : &str) -> HashMap<char, i32>{
    let mut counter = HashMap::new();
    for c in s.chars(){
        *counter.entry(c).or_insert(0) += 1;
    }
    return counter;
}