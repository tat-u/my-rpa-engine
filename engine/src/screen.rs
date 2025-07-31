use crate::structs::VecWH;

#[unsafe(no_mangle)]
pub extern "system" fn get_scr_wh() -> VecWH {
    use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

    unsafe {
        VecWH {
            w: GetSystemMetrics(SM_CXSCREEN),
            h: GetSystemMetrics(SM_CYSCREEN),
        }
    }
}
