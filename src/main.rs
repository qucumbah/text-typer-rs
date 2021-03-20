use winapi::um::winuser::*;

fn main() {
    while unsafe { GetAsyncKeyState(VK_INSERT) } == 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // press_button('a');
    // press_button('A');

    press_key('a');
    press_key('A');
    press_key(':');
    press_key('R');
    press_key('d');
    // println!("Enter your message: ");

    // let mut user_input = String::new();

    // std::io::stdin()
    //     .read_line(&mut user_input)
    //     .expect("Failed to read a line");

    // println!("Press insert key to insert your message...");

    // while unsafe { GetAsyncKeyState(VK_INSERT) } == 0 {
    //     std::thread::sleep(std::time::Duration::from_millis(100));
    // }

    // let bytes = user_input.trim().as_bytes();

    // println!("{}", std::mem::size_of::<INPUT>());

    // for byte in bytes {
    //     let letter = (*byte) as i8;
    //     let key_virtual_code = unsafe { VkKeyScanA(letter) } as u32;
    //     let key_scan_code = unsafe { MapVirtualKeyW(key_virtual_code, MAPVK_VK_TO_VSC) } as u16;

    //     // println!("{} {}", letter, virtual_key_code);

    //     let keyboard_input = KEYBDINPUT {
    //         wVk: 0,
    //         time: 0,
    //         dwExtraInfo: 0,
    //         wScan: key_scan_code,
    //         dwFlags: 0,
    //     };

    //     let mut input = INPUT {
    //         type_: INPUT_KEYBOARD,
    //         u: unsafe { std::mem::transmute_copy(&keyboard_input) },
    //     };

    //     print_bytes(&input);

    //     unsafe { SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32) };
    // }
}

fn press_key(letter: char) {
    let key_virtual_code = unsafe { VkKeyScanW(letter as u16) } as u32;

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

    // if down {
    //     if key_virtual_code_high & 0x1 != 0 { toggle_key_code(VK_SHIFT as u32, true) } // Shift
    //     if key_virtual_code_high & 0x2 != 0 { toggle_key_code(VK_MENU as u32, true) } // Ctrl
    //     if key_virtual_code_high & 0x4 != 0 { toggle_key_code(VK_CONTROL as u32, true) } // Alt

    //     toggle_key_code(key_virtual_code_low, true);
    // } else {
    //     toggle_key_code(key_virtual_code_low, false);

    //     if key_virtual_code_high & 0x1 != 0 { toggle_key_code(VK_SHIFT as u32, false) } // Shift
    //     if key_virtual_code_high & 0x2 != 0 { toggle_key_code(VK_MENU as u32, false) } // Ctrl
    //     if key_virtual_code_high & 0x4 != 0 { toggle_key_code(VK_CONTROL as u32, false) } // Alt
    // }
}

fn toggle_key_code(key_virtual_code: u32, down: bool) {
    let key_scan_code = unsafe { MapVirtualKeyA(key_virtual_code & 0xff, MAPVK_VK_TO_VSC) } as u16;

    println!("{} {}", key_virtual_code, key_scan_code);

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

    // print_bytes(&input);

    unsafe { SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32) };
}

// fn press_button(letter: char) {
//     let key_virtual_code = unsafe { VkKeyScanW(letter as u16) } as u32;

//     let key_virtual_code_low = key_virtual_code & 0xff;

//     let key_scan_code = unsafe { MapVirtualKeyA(key_virtual_code_low, MAPVK_VK_TO_VSC) } as u16;

//     println!("{} {}", key_virtual_code, key_scan_code);

//     let key_virtual_code_high = key_virtual_code >> 8 & 0xff;

//     let keyboard_input = KEYBDINPUT {
//         wVk: 0,
//         time: 0,
//         dwExtraInfo: 0,
//         wScan: key_scan_code,
//         dwFlags: KEYEVENTF_SCANCODE,
//     };

//     let mut input = INPUT {
//         type_: INPUT_KEYBOARD,
//         u: unsafe { std::mem::transmute_copy(&keyboard_input) },
//     };

//     // print_bytes(&input);

//     unsafe { SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32) };
// }

// fn print_bytes<T: Sized>(value: &T) {
//     unsafe {
//         let bytes = any_as_u8_slice(value);
//         for byte in bytes {
//             println!("{}", byte);
//         }
//     }
// }

// unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
//     ::std::slice::from_raw_parts(
//         (p as *const T) as *const u8,
//         ::std::mem::size_of::<T>(),
//     )
// }
