use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let who = if args.len() > 1 {
        String::from(&args[1])
    } else {
        String::from("world")
    };
    Command::new("cowsay")
        .arg(format!("Hello, {}!", who))
        .spawn()
        .expect("failed to execute process");
}
