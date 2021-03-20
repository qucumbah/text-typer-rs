use winapi::um::winuser::*;

fn main() {
    print!("Enter your message: ");

    {
        use std::io::Write;
        std::io::stdout().flush().expect("Should have flushed successfully");
    }

    let mut user_input = String::new();

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read a line");

    println!("Great, we've got your message!");
    println!("Now switch to the target window and press the insert key to insert your message...");

    while unsafe { GetAsyncKeyState(VK_INSERT) } == 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let letters: Vec<char> = user_input.trim().chars().collect();

    for letter in letters {
        press_key(letter);
    }
}

fn press_key(key: char) {
    let key_virtual_code = unsafe { VkKeyScanW(key as u16) } as u32;

    let key_virtual_code_low = key_virtual_code & 0xff;
    let key_virtual_code_high = key_virtual_code >> 8 & 0xff;

    if key_virtual_code_high & 0x1 != 0 { toggle_key_code(VK_SHIFT as u32, true) } // Shift
    if key_virtual_code_high & 0x2 != 0 { toggle_key_code(VK_MENU as u32, true) } // Ctrl
    if key_virtual_code_high & 0x4 != 0 { toggle_key_code(VK_CONTROL as u32, true) } // Alt

    toggle_key_code(key_virtual_code_low, true);
    toggle_key_code(key_virtual_code_low, false);

    if key_virtual_code_high & 0x1 != 0 { toggle_key_code(VK_SHIFT as u32, false) } // Shift
    if key_virtual_code_high & 0x2 != 0 { toggle_key_code(VK_MENU as u32, false) } // Ctrl
    if key_virtual_code_high & 0x4 != 0 { toggle_key_code(VK_CONTROL as u32, false) } // Alt
}

fn toggle_key_code(key_virtual_code: u32, down: bool) {
    let key_scan_code = unsafe { MapVirtualKeyA(key_virtual_code & 0xff, MAPVK_VK_TO_VSC) } as u16;

    let keyboard_input = KEYBDINPUT {
        wVk: 0,
        time: 0,
        dwExtraInfo: 0,
        wScan: key_scan_code | if down { 0 } else { 0x80 },
        dwFlags: KEYEVENTF_SCANCODE | if down { 0 } else { KEYEVENTF_KEYUP },
    };

    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { std::mem::transmute_copy(&keyboard_input) },
    };

    unsafe { SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32) };
}
