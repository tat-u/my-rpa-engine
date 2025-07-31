#[unsafe(no_mangle)]
pub extern "system" fn get_cur_spd() -> i32 {
    use winapi::um::winuser::{SPI_GETMOUSESPEED, SystemParametersInfoW};

    let mut speed: i32 = 0;

    unsafe {
        SystemParametersInfoW(
            SPI_GETMOUSESPEED,
            0,
            &mut speed as *mut _ as *mut _,
            0, // don't change the setting, so set to 0
        );
    };

    speed
}
