use std::fs::File;
use std::io::Write;

fn main() {
    // Geenrate a key
    let key = actix_web::cookie::Key::generate();
    let key = key.master();

    // Write it to the file
    let mut f = File::create("session_key.txt").unwrap();
    f.write_all(key).unwrap();
}
