use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let who = if args.len() > 1 {
        String::from(&args[1])
    } else {
        String::from("world")
    };
    println!("Hello, {}!", who);
}
