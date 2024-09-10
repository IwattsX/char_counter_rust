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
fn str_to_mapped_char(s : &str) -> HashMap<char, i32>{
    let mut counter = HashMap::new();
    for c in s.chars(){
        *counter.entry(c).or_insert(0) += 1;
    }
    return counter;
}

use std::io;

fn main() {
    println!("Enter a string: ");

    let mut input = String::new();  // Corrected the String::new() syntax
    io::stdin().read_line(&mut input).expect("Failed to read from stdin");

    let resmap = str_to_mapped_char(&input.to_uppercase());

    let mut max = -1;
    let mut char_res = ' ';

    for (key, value) in resmap{
        if value > max {
            max = value;
            char_res = key;
        }
    }

    println!("{}: {}",char_res, max);
}

