/* Including library */
use std::env;

/* Linking with other files */
pub mod grep_display;

/* Main function */
fn main() {
   let args: Vec<String> = env::args().collect();
   let file_path = &args[1];
   grep_display::display(file_path);
}
