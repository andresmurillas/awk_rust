use std::env;
use std::fs;

pub fn get_command() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if &args.len() < &3 {
        eprintln!("awk [prog] [file]");
        return vec!("end".to_string(), "".to_string());
    }

    let prog = &args[1];

    let file: String;

    if fs::metadata(&args[2]).is_ok() {
        file = fs::read_to_string(&args[2]).expect("Should have read from file.");
        let command: Vec<String> = vec![prog.to_string(), file];

        return command;
    } else {
        eprintln!("File {} does not exist in current directory.", &args[2]);
        return vec!("end".to_string(), "".to_string());
    }

}