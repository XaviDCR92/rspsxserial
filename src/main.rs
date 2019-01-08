extern crate serial;
extern crate regex;
#[macro_use] extern crate lazy_static;
mod cmdline;
mod app;
mod transfer;

/// Main function.
fn main() {
    // Read command line arguments.
    match cmdline::process_arguments() {
        None => return,
        Some(hash) => {
            // Execute application logic.
            match app::app(hash) {
                _ => return
            }
        }
    }
}
