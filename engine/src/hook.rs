// ! : This file is not reviewed carefully. Some code may be incorrect or incomplete.

use std::time::Duration;

static mut HHOOK: winapi::shared::windef::HHOOK = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub extern "system" fn hook() -> () {
    use std::ptr::null_mut;
    use winapi::um::winuser::{
        MSG, PM_REMOVE, PeekMessageW, SetWindowsHookExW, UnhookWindowsHookEx, WH_MOUSE_LL,
    };

    unsafe {
        HHOOK = SetWindowsHookExW(WH_MOUSE_LL, Some(callback), null_mut(), 0 as u32);

        let mut msg: MSG = std::mem::zeroed();

        let start = std::time::Instant::now();
        while start.elapsed() < Duration::from_secs(3) {
            if PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) > 0 {
                println!("Message received: {}", msg.message);
            }
        }

        UnhookWindowsHookEx(HHOOK);
    }
}

unsafe extern "system" fn callback(ncode: i32, wparam: usize, lparam: isize) -> isize {
    use winapi::um::winuser::CallNextHookEx;

    println!(
        "Hook called with ncode: {}, wparam: {}, lparam: {}",
        ncode, wparam, lparam
    );

    unsafe { CallNextHookEx(HHOOK, ncode, wparam, lparam) }
}
