// TODO: Consider using GetCursorInfo, GetCursorPos, GetCursor, & GetPhysicalCursorPos

// TODO: pub extern "system" ?

#[unsafe(no_mangle)]
pub extern "stdcall" fn mov_rel(dx: i32, dy: i32) -> i32 {
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
pub extern "stdcall" fn mov_abs_scp(x: i32, y: i32) -> bool {
    use winapi::um::winuser::SetCursorPos;

    unsafe { SetCursorPos(x, y) != 0 }
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn mov_abs_si(x: i32, y: i32) -> i32 {
    use crate::screen::{get_scr_h, get_scr_w};
    use winapi::um::winuser::{
        INPUT, INPUT_MOUSE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSEINPUT, SendInput,
    };

    let scr_w = get_scr_w(); // screen width
    let scr_h = get_scr_h(); // screen height

    let mouse_input = MOUSEINPUT {
        dx: (x * 65536 + scr_w - 1) / scr_w,
        dy: (y * 65536 + scr_h - 1) / scr_h,
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

// FIXME: Below code is not tested yet, needs to be verified
#[repr(C)]
pub struct RetPoint {
    pub x: i32,
    pub y: i32,
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn get_cursor_pos() -> RetPoint {
    use winapi::shared::windef::POINT;
    use winapi::um::winuser::GetCursorPos;

    let mut point = POINT { x: 0, y: 0 };

    unsafe {
        GetCursorPos(&mut point);
    }

    RetPoint {
        x: point.x,
        y: point.y,
    }
}
