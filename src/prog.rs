pub fn parse(prog: &String, file: &String) -> String {

    let mut output: String = "".to_string();

    let split = prog.split(' ').collect::<Vec<&str>>();
    let subs = split[0];
    let word: String = subs.replace("/", "");
    let lines = file.split("\n").collect::<Vec<&str>>();
    for line in lines {
        if line.contains(&word) {
            output.push_str(line);
            output.push_str("\n");
        }
    }

    return output;
}
