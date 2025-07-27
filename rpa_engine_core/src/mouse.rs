use winapi::um::winuser::INPUT;
use winapi::um::winuser::LPINPUT;
use winapi::um::winuser::MOUSEINPUT;
use winapi::um::winuser::SendInput;

#[unsafe(no_mangle)]
pub extern "stdcall" fn mmv(dx: i32, dy: i32) {
    let mouse_input = MOUSEINPUT {
        dx,
        dy,
        mouseData: 0x0,
        dwFlags: 0x0001, // MOUSEEVENTF_MOVE
        time: 0x0,
        dwExtraInfo: 0x0,
    };

    let mut input: INPUT = INPUT {
        type_: 0,                                       // INPUT_MOUSE
        u: unsafe { std::mem::transmute(mouse_input) }, // union
    };

    let cinputs = 1; // Number of inputs to send

    let pinputs: LPINPUT = &mut input as *mut INPUT; // Cast to raw pointer

    let cbsize = std::mem::size_of::<INPUT>() as i32;

    unsafe {
        SendInput(cinputs, pinputs, cbsize);
    }
}
