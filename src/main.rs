use clap::{Parser, ValueEnum};
use std::env;
use std::process::Command;

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
    println!("{hyprland_instance_signature}");

    let args = Cli::parse();

    dbg!(&args);

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
