/// Input transmission layer for windows devices

use std::time::Duration;

use crate::key::{Code, Press};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, VIRTUAL_KEY, 
    KEYBD_EVENT_FLAGS, KEYEVENTF_UNICODE, KEYEVENTF_KEYUP,
    SendInput
};

async fn release_key(mut event: KEYBDINPUT, duration: Duration) {
    tokio::time::sleep(duration).await;

    event.dwFlags |= KEYEVENTF_KEYUP;

    let input_event = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 { ki: event },
    };

    let events = vec![input_event];

    unsafe { SendInput(&events, std::mem::size_of::<INPUT>() as i32); }
}

pub async fn send_inputs(keys: &[Press]) {
    if keys.is_empty() { return; }

    let mut keys_start: Vec<INPUT> = Vec::with_capacity(keys.len());
    let mut keys_end = Vec::with_capacity(keys.len());

    let mut prev = &keys[1];

    for key in keys.iter() {
        // Decode the keypress as a virtual and unicode key
        let (flags, vk, uk) = match key.code {
            Code::VirtualKey(k) => (KEYBD_EVENT_FLAGS(0), k, k),
            Code::UnicodeKey(k) => (KEYEVENTF_UNICODE, 0, k)
        };

        let keyboard_event = KEYBDINPUT {
            wVk: VIRTUAL_KEY(vk),
            wScan: uk,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0
        };

        let input_event = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { ki: keyboard_event },
        };

        // Add future to list of key-up events to join later
        keys_end.push(release_key(keyboard_event, key.duration));

        // Inject an additional keydown event
        if prev.code == key.code {
            let mut up_event = keyboard_event;
            up_event.dwFlags |= KEYEVENTF_KEYUP;

            let up_event = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 { ki: up_event },
            };

            keys_start.push(up_event);
        }

        // Add to the list of key-down events
        keys_start.push(input_event);

        prev = key;
    }

    // Send the initial KEY_DOWN events
    unsafe { SendInput(&keys_start, std::mem::size_of::<INPUT>() as i32); }

    // Wait and send the following KEY_UP events
    futures::future::join_all(keys_end).await;
}