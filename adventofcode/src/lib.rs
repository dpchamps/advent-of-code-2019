use std::fs;
use std::io;

pub fn read_input_file(name : &str) -> String{
    // panic if not found...
    return fs::read_to_string(format!("resources/{}", name)).unwrap()
}
