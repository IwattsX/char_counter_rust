use std::io;

use high_char_occurance::str_to_mapped_char;



/// The driver code, ask he user for a string input then prints the most occuring character in that string 
fn main() {
    println!("Enter a string: ");

    let mut input = String::new();  // Corrected the String::new() syntax
    io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    
    input = input.to_uppercase();
    let resmap = str_to_mapped_char(&input);

    let mut max = -1;
    let mut char_res = ' ';

    for (key, value) in resmap{
        if value > max {
            max = value;
            char_res = key;
        }
    }
    // example output: 'L' 2
    println!("\'{}\' {}",char_res, max);
}

