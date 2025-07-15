use std::fmt::Display;

trait Loggable {
    fn log(&self);
}

impl<T: Display> Loggable for T {
    fn log(&self) {
        println!("[LOG] {}", self);
    }
}

fn main() {
    let x = 22;
    x.log();
}
