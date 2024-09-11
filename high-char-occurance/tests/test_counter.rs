extern crate high_char_occurance; // Import the library
use high_char_occurance::str_to_mapped_char;

use maplit::hashmap;
use std::collections::HashMap;


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn str_to_mapped_char_hello() {
        let map: HashMap<char, i32> = hashmap! {
            'H' => 1,
            'E' => 1,
            'L' => 2,
            'O' => 1,
        };
        assert_eq!(str_to_mapped_char("HELLO"), map);
    }

    
    #[test]
    fn str_to_mapped_char_berries(){
        let map: HashMap<char, i32> = hashmap! {
            'B' => 1,
            'E' => 2,
            'R' => 2,
            'I' => 1,
            'S' => 1,
        };
        assert_eq!(str_to_mapped_char("BERRIES"), map);
    }

    // main function integration test
    #[test]
    fn test_main_function() {
        let mut child = Command::new(env!("CARGO_BIN_EXE_high-char-occurance"))
            .stdin(Stdio::piped())     // Allow writing to stdin
            .stdout(Stdio::piped())    // Capture stdout
            .spawn()
            .expect("Failed to execute process");

        // Simulate user input "hello"
        let input = "hello\n";
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");

        // Capture the output
        let output = child.wait_with_output().expect("Failed to read stdout");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("L: 2"));  // Check if output contains the expected result
    }
}

use std::process::{Command, Stdio};
use std::io::Write; // To write to stdin


