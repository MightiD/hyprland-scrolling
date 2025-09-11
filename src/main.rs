use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::env;
use std::process::Command;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use serde_json;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    MoveWindow,
    MoveFocus,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(required = true)]
    mode: Mode,
    #[arg(required = true)]
    direction: Direction,
}

#[derive(Debug, Deserialize)]
struct Config {
    groups: Vec<Vec<u8>>,
}

fn get_current_workspace() -> String {
    let command = Command::new("hyprctl")
        .arg("activeworkspace")
        .output()
        .expect("Couldn't get current workspace");
    let s = match String::from_utf8(command.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid command output: {}", e),
    };

    //this is a terrible way of doing this probably
    //also 2nd item prolly wont work all the time
    String::from(s.split_whitespace().nth(2).unwrap())
}

fn move_window(target: i8) {
    Command::new("hyprctl")
        .arg("dispatch")
        .arg("movetoworkspace") //move window to workspace
        .arg(target.to_string())
        .spawn()
        .expect("Failed to run hyprctl");
}

fn move_focus(target: i8) {
    Command::new("hyprctl")
        .arg("dispatch")
        .arg("workspace") //move focus to workspace
        .arg(target.to_string())
        .spawn()
        .expect("Failed to run hyprctl");
}

//use command hyprclt dispatch movetoworkspace {workspace} for this all to work

fn main() {
    let hyprland_instance_signature =
        env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("Hyprland must be running");
    println!("Hyprland Instance Signature: {hyprland_instance_signature}");

    let home_dir = env::var("HOME").expect("Home directory not set");

    let args = Cli::parse();

    let path = PathBuf::from(home_dir).join(".config/hypr/scrolling.json");

    let mut file = File::open(path)
        .expect("Couldn't open file at ~/.config/hypr/scrolling.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let config: Config = serde_json::from_str(&contents)
        .expect("Couldn't parse config file at ~/.config/hypr/scrolling.json");

    dbg!(&config);

    let mut current_workspace: i8 = get_current_workspace().parse().expect("Not a valid number");
    match args.direction {
        Direction::Up => current_workspace += 1,
        Direction::Down => current_workspace -= 1,
    }
    
    match args.mode {
        Mode::MoveWindow => move_window(current_workspace),
        Mode::MoveFocus => move_focus(current_workspace),
    }
    println!("{}", current_workspace);

}
