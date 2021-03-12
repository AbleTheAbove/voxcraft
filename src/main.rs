pub const ROOT_PATH: &str = ".rustcraft";

mod addons;
mod startup;

fn main() {
    addons::load_addons();
    if startup::addon_folder_exists() {
        println!("Good to go!");
    } else {
        println!("Rustcraft folder can't be found.")
    }
}
