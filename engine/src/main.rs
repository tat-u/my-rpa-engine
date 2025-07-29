mod mouse;
mod screen;
mod structs;

fn main() {
    {
        mouse::mov_abs_scp(555, 555);
        let r = mouse::get_cur_pos();
        println!("Mouse position should be (555, 555): ({}, {})", r.x, r.y);
    };
    {
        mouse::mov_abs_si(555, 555);
        let r = mouse::get_cur_pos();
        println!("Mouse position should be (555, 555): ({}, {})", r.x, r.y);
    };
    {
        // Mouse speed affects the movement
        mouse::mov_abs_scp(0, 0);
        mouse::mov_rel(100, 100);
        let r = mouse::get_cur_pos();
        println!("Mouse position: ({}, {})", r.x, r.y);
    };
}
