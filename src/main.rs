pub const ROOT_PATH: &str = ".rustcraft";

mod addons;
mod config;
mod startup;

fn main() {
    let mut config = config::load_config();
    // dont leave this in
    config.height -= 1;
    println!("{:?}", config);
    addons::load_addons();
    if startup::addon_folder_exists() {
        println!("Good to go!");
    } else {
        println!("Rustcraft folder can't be found.")
    }
}
