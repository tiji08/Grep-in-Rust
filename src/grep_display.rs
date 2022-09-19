use std::fs;

/* Public display function */
pub fn display(file_path : &str) {
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("No file present, please create file");
    println!("With text:\n {}", contents);
}
