use std::env;

pub fn get_command() {
    let args: Vec<String> = env::args().collect();

    println!("{:?} ", args);
}