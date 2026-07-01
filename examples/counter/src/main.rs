use num::Num;
use pinions::{Wid, Win};
use std::fmt::Write;
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowId},
};

// Define our window type with specific generic parameters
// T: title length (10 for "Counter")
// E: event length (32)
// V: widget count (1)
// L: label length (10 -  10 (enough for a counter)
// I: icon type (u8, which implements Num)
// S: icon size (0 - no icon)
type MyWin = Win<10, 32, 1, 10, u8, 0>;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut win = MyWin::new();
    // Set the window title
    win.title.clear();
    let _ = write!(win.title, "Counter");

    // Create a widget and set its initial label to "0"
    let mut widget = Wid::<10, u8, 0, 32>::new(); // L=10, I=u8, S=0, E=32
    widget.label.clear();
    let _ = write!(widget.label, "{}", win.count); // Initial count is 0

    // Add the widget to the window's widget vector
    win.widgets.push(widget);

    // Run the event loop
    event_loop.run_app(&mut win).unwrap();
}
