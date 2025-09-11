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

fn move_window(target: u8) {
    Command::new("hyprctl")
        .arg("dispatch")
        .arg("movetoworkspace") //move window to workspace
        .arg(target.to_string())
        .spawn()
        .expect("Failed to run hyprctl");
}

fn move_focus(target: u8) {
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

    let mut current_workspace: u8 = get_current_workspace().parse().expect("Not a valid number");

    let mut target_workspace: u8 = current_workspace;

    let mut pos: (usize, usize) = {
        let mut result = (0, 0);
        let mut found = false;
        for (i, group) in config.groups.iter().enumerate() {
            for (j, workspace) in group.iter().enumerate() {
                if *workspace as usize == current_workspace as usize {
                    result = (i, j);
                    found = true;
                    break;
                }
            }
        }
        if !found{
            eprintln!("Couldnt find current workspace in configuration");
            std::process::exit(1);
        }
        result
    };

    match args.direction {
        Direction::Up => {
            //i cant be bothered to fix the errors that come with just out of bounds indexing
            //it isnt meant to do anything anyway if the index is out of bounds
            pos.1 += 1;
            target_workspace = match config.groups[pos.0].get(pos.1) {
                Some(ws) => *ws,
                None => {
                    eprintln!("No higher workspace to move to on this display");
                    std::process::exit(0);
                }
            };
        },
        Direction::Down => {
            pos.1 -= 1;
            target_workspace = match config.groups[pos.0].get(pos.1) {
                Some(ws) => *ws,
                None => {
                    eprintln!("No higher workspace to move to on this display");
                    std::process::exit(0);
                }
            };
        },
    }
    
    match args.mode {
        Mode::MoveWindow => move_window(target_workspace),
        Mode::MoveFocus => move_focus(target_workspace),
    }

}
