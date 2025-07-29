// TODO: Consider using GetCursorInfo, GetCursorPos, GetCursor, & GetPhysicalCursorPos

// TODO: pub extern "system" ?

#[unsafe(no_mangle)]
pub extern "stdcall" fn mov(dx: i32, dy: i32) -> i32 {
    use winapi::um::winuser::{INPUT, MOUSEINPUT, SendInput};

    let mouse_input = MOUSEINPUT {
        dx,
        dy,
        mouseData: 0x0,
        dwFlags: 0x0001, // MOUSEEVENTF_MOVE
        time: 0x0,
        dwExtraInfo: 0x0,
    };
    let mut input = INPUT {
        type_: 0,                                       // INPUT_MOUSE
        u: unsafe { std::mem::transmute(mouse_input) }, // union
    };

    let cinputs = 1 as u32; // Number of inputs to send
    let pinputs = &mut input as *mut INPUT;
    let cbsize = std::mem::size_of::<INPUT>() as i32;

    unsafe { SendInput(cinputs, pinputs, cbsize) as i32 }
}

// https://qiita.com/kob58im/items/23df9e22778b33986d1c#44-%E7%B5%90%E8%AB%96%E6%AD%A3%E7%A2%BA%E3%81%AB%E5%A4%89%E6%8F%9B%E3%81%A7%E3%81%8D%E3%82%8B%E3%81%A7%E3%81%82%E3%82%8D%E3%81%86%E3%82%B3%E3%83%BC%E3%83%89

#[unsafe(no_mangle)]
pub extern "stdcall" fn mov_to(x: i32, y: i32) -> i32 {
    use crate::screen::{get_scr_h, get_scr_w};
    use winapi::um::winuser::{
        INPUT, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSEINPUT, SendInput,
    };

    let scr_w = get_scr_w(); // screen width
    let scr_h = get_scr_h(); // screen height

    let mouse_input = MOUSEINPUT {
        dx: (x * 65536 + scr_w - 1) / scr_w,
        dy: (y * 65536 + scr_h - 1) / scr_h,
        mouseData: 0x0,
        dwFlags: MOUSEEVENTF_ABSOLUTE + MOUSEEVENTF_MOVE,
        time: 0x0,
        dwExtraInfo: 0x0,
    };
    let mut input = INPUT {
        type_: 0,                                       // INPUT_MOUSE
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
