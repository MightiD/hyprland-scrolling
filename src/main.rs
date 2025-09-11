use std::env;
use std::process::Command;

fn getCurrentWorkspace() -> String {
    let command = Command::new("hyprctl")
        .arg("activeworkspace")
        .output()
        .expect("Couldn't get current workspace");
    let s = match String::from_utf8(command.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid command output: {}", e)
    };

    String::from(s.split_whitespace().nth(2).unwrap())

}

//use command hyprclt dispatch movetoworkspace {workspace} for this all to work

fn main() {
    let hyprland_instance_signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .expect("Hyprland must be running");
    println!("{hyprland_instance_signature}");

    let current_workspace = getCurrentWorkspace();
    println!("{}", current_workspace);

}
