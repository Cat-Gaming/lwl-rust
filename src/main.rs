use lwl::LWL;


pub mod lwl;

fn main() {
    let mut win = LWL::init_window(200, 200, "Testing...");
    while !win.should_close() {
        win.poll_events();
    }
}