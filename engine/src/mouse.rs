// TODO: Consider using GetCursorInfo, GetCursorPos, GetCursor, & GetPhysicalCursorPos

#[unsafe(no_mangle)]
pub extern "stdcall" fn mmv(dx: i32, dy: i32) {
    use winapi::um::winuser::{INPUT, MOUSEINPUT, SendInput};

    let cinputs = 1; // Number of inputs to send
    let pinputs = {
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

        &mut input as *mut INPUT
    };
    let cbsize = std::mem::size_of::<INPUT>();

    unsafe {
        SendInput(cinputs, pinputs, cbsize as i32);
    }
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
