use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::pass::ClearColor,
};

mod addons;
mod config;
mod startup;
mod world;
use world::chunk::Chunk;
mod input;

pub const ROOT_PATH: &str = "assets";
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
    } //println!("{}",

    {
        // NOTE(Able): Prerendering work on chunks
        let chunk_data = [[[0; 32]; 32]; 32];
        let chunk = Chunk { data: chunk_data };
        chunk.fetch(0, 1, 2);
    }
    //https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs
    App::build()
        .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_resource(WindowDescriptor {
            title: game_meta_bar,
            width: config.width as f32,
            height: config.height as f32,
            vsync: config.vsync,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(input::toggle_cursor.system())
        .add_system(input::keyboard_input_system.system())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(text_update_system.system())
        .run();
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        // 2d camera
        .spawn(CameraUiBundle::default())
        // texture
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: asset_server.load("FiraSans-Regular.ttf"),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(FpsText);
}
