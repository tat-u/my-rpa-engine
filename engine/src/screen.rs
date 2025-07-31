use crate::structs::Vec2;

#[unsafe(no_mangle)]
pub extern "system" fn get_scr_wh() -> Vec2 {
    use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

    unsafe { Vec2::new(GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) }
}
