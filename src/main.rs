mod terminal;
mod prog;

fn main() {
    let command: Vec<String> = terminal::get_command();

    let progs = &command[0];
    let file = &command[1];

    println!("{}", prog::parse(&progs, &file));
}
