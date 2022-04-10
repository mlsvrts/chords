use std::time::Duration;
use crate::VirtualKey;

/// Keyboard events can be unicode-renderable characters (like 'A'), but can
/// also take the form of virutal key events (like 'ESC').
/// 
/// This enum encapsulates the two, in order to allow the underlying driver
/// to diffrentiate between otherwise identical codes.
#[derive(Debug, PartialEq)]
pub enum Code {
    VirtualKey(u16),
    UnicodeKey(u16)
}

/// Keypress events are virtual or unicode key events, over some duration
/// 
/// If duration is `None` the press will be insterted into the input stream as a keydown
/// immediately followed by a keyup.
#[derive(Debug, PartialEq)]
pub struct Press {
    pub code: Code,
    pub duration: Option<Duration>,
}

impl Press {
    pub fn new(code: Code, duration: Option<Duration>) -> Self {
        Self { code, duration }
    }

    /// Create a new unicode-type key event
    pub fn new_unicode(code: impl Into<u16>, duration: Option<Duration>) -> Self {
        Self { code: Code::UnicodeKey(code.into()), duration }
    }

    /// Create a new unicode-type key event for `ms` milliseconds
    pub fn new_unicode_ms(code: impl Into<u16>, ms: u64) -> Self {
        Self::new_unicode(code, Some(Duration::from_millis(ms)))
    }

    /// Produce a new virtual-type key event
    pub fn new_virtual(code: impl Into<u16>, duration: Option<Duration>) -> Self {
        Self { code: Code::VirtualKey(code.into()), duration }
    }

    /// Produce a new virtual-type key event for `ms` milliseconds
    pub fn new_virtual_ms(code: impl Into<u16>, ms: u64) -> Self {
        Self::new_virtual(code, Some(Duration::from_millis(ms)))
    }

    // TODO: Add a send function that transmits the press. Support: Press::from(...).send()
}

/// Helper used when generating key presses from utf16-encoded strings
impl<T: Into<u16>> From<T> for Press {
    fn from(code: T) -> Self {
        Self::new_unicode(code, None)
    }
}

/// Helper used when generating key presses from virtual key-codes
impl From<VirtualKey> for Press {
    fn from(code: VirtualKey) -> Self {
        Self::new_virtual(code as u16, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypress_from_utf16() {
        let expected = Press::new(Code::UnicodeKey(0x0061), None);
        let unicode_a = "a".encode_utf16().next().unwrap();

        let actual = Press::from(unicode_a);

        assert_eq!(actual, expected);
    }
}