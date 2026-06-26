pub use winit;

pub struct App {
    title: String,
}

impl App {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }

    pub fn run(&self) {
        println!("GUI '{}' started from lib Pincers!", self.title);
    }
}
