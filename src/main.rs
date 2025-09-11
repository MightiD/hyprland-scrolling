use std::env;
use std::process::Command;

fn get_current_workspace() -> String {
    let command = Command::new("hyprctl")
        .arg("activeworkspace")
        .output()
        .expect("Couldn't get current workspace");
    let s = match String::from_utf8(command.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid command output: {}", e)
    };

    //this is a terrible way of doing this probably
    //also 2nd item prolly wont work all the time
    String::from(s.split_whitespace().nth(2).unwrap())

}

//use command hyprclt dispatch movetoworkspace {workspace} for this all to work

fn main() {
    let hyprland_instance_signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .expect("Hyprland must be running");
    println!("{hyprland_instance_signature}");

    let mut current_workspace: i8 = get_current_workspace().parse().expect("Not a valid number");
    current_workspace += 1;
    println!("{}", current_workspace);

    Command::new("hyprctl")
        .arg("dispatch")
        //.arg("movetoworkspace") //move window to workspace
        .arg("workspace") //move focus to workspace
        .arg(current_workspace.to_string())
        .spawn()
        .expect("Failed to run hyprctl");
}
