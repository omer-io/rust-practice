use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    static string: &str = "Hello from RUST!";

    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // let mut s = String::new();
    match file.write_all(string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

}