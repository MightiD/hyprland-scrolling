use std::env;

fn main() {
    let hyprland_instance_signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .expect("Hyprland must be running");
    println!("{hyprland_instance_signature}")
}
