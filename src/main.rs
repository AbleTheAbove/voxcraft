use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::pass::ClearColor};

mod addons;
mod config;
mod world;
use world::chunk::Chunk;
mod input;
mod ui;

pub const ROOT_PATH: &str = "assets";
pub const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() {
    let game_meta_bar = format!("Rustcraft ver{}", GAME_VERSION);
    let mut config = config::load_config();
    // dont leave this in
    config.height -= 1 + -1; // weird math to make the compiler not throw a mutable warning
    let font_size = 14 * config.ui_scale;
    addons::load_addons();

    {
        // NOTE(Able): Prerendering work on chunks
        let chunk_data = [[[0; 32]; 32]; 32];
        let chunk = Chunk { data: chunk_data };
        chunk.fetch(0, 1, 2);
    }
    App::build() // TODO(Able):
        .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9))) // NOTE(Able): Clears the window to
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
        .add_system(ui::text_update_system.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    audio: Res<Audio>,
) {
    let music = asset_server.load("asset_pack/steps.mp3"); // TODO(Able): Replace with a good foot step sound/s
    audio.play(music);

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
                font: asset_server.load("asset_pack/FiraSans-Regular.ttf"),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(ui::FpsText)
        /*
         */
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-10.0, 9.5, 5.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });

    let mut iter = 0.0;
    let mut d = 0.0;
    for _x in 0..10 {
        commands
            // cube
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(d, 0.0, 0.0).into()),
                transform: Transform::from_translation(Vec3::new(iter, 0.5, 0.0)),
                ..Default::default()
            });
        d += 0.1;
        iter += 1.0;
    }
}
