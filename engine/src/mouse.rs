// TODO: Consider using GetCursorInfo, GetCursorPos, GetCursor, & GetPhysicalCursorPos

use crate::structs::Vec2;
use core::panic;

#[unsafe(no_mangle)]
pub extern "system" fn btn(button: u32, down: bool) -> i32 {
    use std::mem::size_of;
    use winapi::um::winuser::{
        INPUT, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
        MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEINPUT, SendInput,
    };

    let dwflags: u32 = match down {
        true => match button {
            1 => MOUSEEVENTF_LEFTDOWN,
            2 => MOUSEEVENTF_RIGHTDOWN,
            3 => MOUSEEVENTF_MIDDLEDOWN,
            _ => panic!("Invalid button: {}", button),
        },
        false => match button {
            1 => MOUSEEVENTF_LEFTUP,
            2 => MOUSEEVENTF_RIGHTUP,
            3 => MOUSEEVENTF_MIDDLEUP,
            _ => panic!("Invalid button: {}", button),
        },
    };

    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0x0,
        dwFlags: dwflags,
        time: 0x0,
        dwExtraInfo: 0x0,
    };
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::transmute(mouse_input) },
    };

    let pinputs = &mut input as *mut INPUT;

    unsafe { SendInput(1, pinputs, size_of::<INPUT>() as i32) as i32 }
}

#[unsafe(no_mangle)]
pub extern "system" fn mov_rel(dx: i32, dy: i32) -> i32 {
    use winapi::um::winuser::{INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEINPUT, SendInput};

    let mouse_input = MOUSEINPUT {
        dx,
        dy,
        mouseData: 0x0,
        dwFlags: MOUSEEVENTF_MOVE,
        time: 0x0,
        dwExtraInfo: 0x0,
    };
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::transmute(mouse_input) }, // union
    };

    let cinputs = 1 as u32; // Number of inputs to send
    let pinputs = &mut input as *mut INPUT;
    let cbsize = std::mem::size_of::<INPUT>() as i32;

    unsafe { SendInput(cinputs, pinputs, cbsize) as i32 }
}

#[unsafe(no_mangle)]
pub extern "system" fn mov_abs_scp(x: i32, y: i32) -> bool {
    use winapi::um::winuser::SetCursorPos;

    unsafe { SetCursorPos(x, y) != 0 }
}

#[unsafe(no_mangle)]
pub extern "system" fn mov_abs_si(x: i32, y: i32) -> i32 {
    use crate::screen::get_scr_wh;
    use winapi::um::winuser::{
        INPUT, INPUT_MOUSE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSEINPUT, SendInput,
    };

    let wh = get_scr_wh();
    let w = wh.w;
    let h = wh.h;

    let mouse_input = MOUSEINPUT {
        dx: (x * 65536 + w - 1) / w,
        dy: (y * 65536 + h - 1) / h,
        mouseData: 0x0,
        dwFlags: MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE,
        time: 0x0,
        dwExtraInfo: 0x0,
    };
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::transmute(mouse_input) }, // union
    };

    let cinputs = 1 as u32; // Number of inputs to send
    let pinputs = &mut input as *mut INPUT;
    let cbsize = std::mem::size_of::<INPUT>() as i32;

    unsafe { SendInput(cinputs, pinputs, cbsize) as i32 }
}

#[unsafe(no_mangle)]
pub extern "system" fn clk(button: u32) -> i32 {
    use std::mem::size_of;
    use winapi::um::winuser::{
        INPUT, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
        MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEINPUT, SendInput,
    };

    let dwflags1: u32 = match button {
        1 => MOUSEEVENTF_LEFTDOWN,
        2 => MOUSEEVENTF_RIGHTDOWN,
        3 => MOUSEEVENTF_MIDDLEDOWN,
        _ => panic!("Invalid button: {}", button),
    };
    let dwflags2: u32 = match button {
        1 => MOUSEEVENTF_LEFTUP,
        2 => MOUSEEVENTF_RIGHTUP,
        3 => MOUSEEVENTF_MIDDLEUP,
        _ => panic!("Invalid button: {}", button),
    };

    let mouse_input_1 = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0x0,
        dwFlags: dwflags1,
        time: 0x0,
        dwExtraInfo: 0x0,
    };
    let mouse_input_2 = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0x0,
        dwFlags: dwflags2,
        time: 0x0,
        dwExtraInfo: 0x0,
    };
    let mut inputs = [
        INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::transmute(mouse_input_1) },
        },
        INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::transmute(mouse_input_2) },
        },
    ];

    let pinputs = &mut inputs as *mut INPUT;

    unsafe { SendInput(2, pinputs, size_of::<INPUT>() as i32) as i32 }
}

#[unsafe(no_mangle)]
pub extern "system" fn get_cur_pos() -> Vec2 {
    use winapi::shared::windef::POINT;
    use winapi::um::winuser::GetCursorPos;

    let mut point = POINT { x: 0, y: 0 };

    unsafe {
        GetCursorPos(&mut point);
    }

    Vec2 {
        x: point.x,
        y: point.y,
    }
}
