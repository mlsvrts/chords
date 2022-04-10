use std::time::Duration;
use std::io::{stdout, stdin, Write};

use chords::{Chord, VirtualKey::Enter};

#[tokio::main]
async fn main() {
    // Create a new chord from a string of utf16-characters
    let mut chord = Chord::from("Hello, world!".encode_utf16());

    // Add an enter-key at the end, so that our input will complete
    chord.push_vk(Enter);

    // Wait some time before playing the keys back
    tokio::task::spawn(chord.play_after(Duration::from_millis(1000)));

    // Read the input, and check our result
    print!("Robot ?: ");
    let _ = stdout().flush();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line!");
    
    let input = input.trim();

    assert_eq!(input, "Hello, world!");

    println!("Robot said, '{}'", input);
}