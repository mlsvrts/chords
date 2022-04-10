//! `chords`: A cross-platform keyboard event library
//! 
//! A `Chord` is a set of `key::Press` events that will be sent to the system.
//! Each key press event consists of sending a unicode or virutal KEY_DOWN event,
//! waiting for the specified press duration, and then sending KEY_UP.
//! 
//! The primary focus of `chords` is to  make it as simple as possible to simulate
//! keyboard inputs. For instance, consider this `Chord` that sends 'Hello, world!':
//! 
//! ```no_run
//! # use std::time::Duration;
//! # use std::io::stdin;
//! use chords::{Chord, VirtualKey::Enter};
//! 
//! #[tokio::main]
//! async fn main() {
//!     // Create a new chord from a string of utf16-characters
//!     let mut chord = Chord::from("Hello, world!".encode_utf16());
//! 
//!     // Add an enter-key at the end, so that our input will complete
//!     chord.push_vk(Enter);
//! 
//!     // Wait some time before playing the keys back
//!     tokio::task::spawn(chord.play_after(Duration::from_millis(10)));
//! 
//!     // Read the input, and check our result
//!     let mut input = String::new();
//!     stdin().read_line(&mut input).expect("Failed to read line!");
//! 
//!     assert_eq!(input.trim(), "Hello, world!")
//! }
//! ```


/// Support sending input events on windows platform, via the `SendInput` API
#[cfg(target_os = "windows")]
mod win;

/// Provides a `Press` type, that respresents pressing a key for some duration.
///
/// `Press` events are used to a sequence of key-down + key-up events when playing
/// a `Chord` of keypresses.
pub mod key;

/// Provides the list of virtual key codes on windows/unix machines
pub mod codes;
pub use codes::VirtualKey; 


/// A `Chord` is a group of key-presses that will be transmitted in-bulk to the system
pub struct Chord {
    pub keys: Vec<key::Press>,
}

/// Helper implementation to generate a `Chord` from vectors of unicode characters.
/// 
/// This supports the syntax `Chord::from("Hello!".encode_utf16()).
impl<I: Into<u16>, T: IntoIterator<Item = I>> From<T> for Chord {
    fn from(codes: T) -> Self {
        Self {
            keys: codes.into_iter().map(|k| key::Press::from(k)).collect()
        }
    }
}

impl Chord {
    /// Append a virtual keypress to the end of the chord
    pub fn push_vk(&mut self, key: VirtualKey) {
        self.keys.push(key::Press::from(key))
    }

    /// Playback the key events after some delay
    pub async fn play_after(self, duration: std::time::Duration) {
        tokio::time::sleep(duration).await;
        self.play().await
    }

    /// Playback the key events to the system
    pub async fn play(&self) {
        #[cfg(target_os = "windows")]
        win::send_inputs(&self.keys).await
    }
}