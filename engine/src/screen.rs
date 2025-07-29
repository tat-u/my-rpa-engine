#[unsafe(no_mangle)]
pub extern "system" fn get_scr_w() -> i32 {
    use winapi::um::winuser::GetSystemMetrics;

    unsafe { GetSystemMetrics(0) } // SM_CXSCREEN
}

#[unsafe(no_mangle)]
pub extern "system" fn get_scr_h() -> i32 {
    use winapi::um::winuser::GetSystemMetrics;

    unsafe { GetSystemMetrics(1) } // SM_CYSCREEN
}
