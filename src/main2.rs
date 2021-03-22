// We will be trying to minimize code in main.rs

mod addons;
mod config;
mod startup;
mod world;
use world::chunk::Chunk;

pub const ROOT_PATH: &str = ".rustcraft";
pub const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() {
    let root = startup::get_root();
    let game_meta_bar = format!("Rustcraft ver{}", GAME_VERSION);
    let mut config = config::load_config();
    // dont leave this in
    config.height -= 1 + -1; // weird math to make the compiler not throw a mutable warning
    let font_size = 14 * config.ui_scale;
    addons::load_addons();

    let text;
    if startup::addon_folder_exists() {
        text = "Addon folder found!"; // TODO(*): Sensible names for these variables
    } else {
        text = "Rustcraft folder can't be found.";
    }
    let assets = root.join("assets");

    {
        // NOTE(Able): Prerendering work on chunks
        let chunk_data = [[[0; 32]; 32]; 32];
        let chunk = Chunk { data: chunk_data };
        chunk.fetch(0, 1, 2);
    }

    use bevy::prelude::*;

    App::build().add_system(hello_world.system()).run();
}
struct Person;
struct Name(String);
fn add_people(commands: &mut Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

fn hello_world() {
    println!("hello world!");
}
