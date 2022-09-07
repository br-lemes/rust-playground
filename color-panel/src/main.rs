use std::env;
use std::process::Command;

fn color(arg: &str) -> &str {
    match arg {
        "green" => "-s 0.200000 -s 0.819608 -s 0.478431 -s 1",
        "orange" => "-s 1 -s 0.470588 -s 0 -s 1",
        "red" => "-s 0.878431 -s 0.105882 -s 0.141176 -s 1",
        "yellow" => "-s 0.964706 -s 0.827451 -s 0.176471 -s 1",
        _ => panic!("Invalid color"),
    }
}

fn set_panel(panel: &str, color: &str) {
    Command::new("xfconf-query")
        .args(
            format!(
                "--channel xfce4-panel -p /panels/{}/background-{}",
                panel, color
            )
            .split(' '),
        )
        .spawn()
        .expect("failed to execute process");
}

fn set_all(color: &str) {
    set_panel("panel-1", color);
    set_panel("panel-2", color);
}

fn main() {
    const RGBA: &str = "rgba --create -t double -t double -t double -t double";
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            set_all(&*format!("{} {}", RGBA, color(&args[1])));
            set_all("style -s 1");
        }
        5 => {
            set_all(&*format!(
                "{} -s {} -s {} -s {} -s {}",
                RGBA, args[1], args[2], args[3], args[4]
            ));
            set_all("style -s 1");
        }
        _ => set_all("style -s 0"),
    }
}
