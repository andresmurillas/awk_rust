pub fn parse(prog: &String, file: &String) -> String {

    let mut output: String = "".to_string();

    let split_prog: Vec<char> = prog.chars().collect();

    let first_char = split_prog[0];

    match first_char {
        '/'  => {
            let word: String = prog.replace("/", "");
            let lines = file.split("\n").collect::<Vec<&str>>();
            for line in lines {
                if line.contains(&word) {
                    output.push_str(line);
                    output.push_str("\n");
                }
            }

            return output;
        },
        '\'' => {
            return "\'".to_string();
        }
        _ => {
            return "reg".to_string();
        }
    }

}
