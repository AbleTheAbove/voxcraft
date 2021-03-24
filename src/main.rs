use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::pass::ClearColor};
mod addons;
mod config;
mod world;
use world::chunk::{Chunk, CHUNK_SIZE};
mod input;
mod ui;

pub const ROOT_PATH: &str = "assets";
pub const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() {
    let game_meta_bar = format!("Rustcraft ver{}", GAME_VERSION);
    let mut config = config::load_config();
    // dont leave this in
    config.height -= 1 + -1; // weird math to make the compiler not throw a mutable warning
    let _font_size = 14 * config.ui_scale;
    addons::load_addons();

    {
        // NOTE(Able): Prerendering work on chunks
        let chunk_data = [[[0; 32]; 32]; 32];
        let chunk = Chunk { data: chunk_data };
        chunk.fetch(0, 1, 2);
    }
    App::build() // TODO(Able):
        .add_startup_system(setup.system())
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
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(input::toggle_cursor.system())
        .add_system(input::keyboard_input_system.system())
        .init_resource::<ui::ButtonMaterials>()
        .add_system(ui::text_update_system.system())
        .add_system(ui::button_system.system())
        .add_system(input::print_mouse_events_system.system())
        .run();
}
#[derive(Debug)]
struct Player {
    x_rot: f32,
}
fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    audio: Res<Audio>,
    button_materials: Res<ui::ButtonMaterials>,
) {
    let music = asset_server.load("asset_pack/steps.mp3"); // TODO(Able): Replace with a good foot step sound/s
    audio.play(music);

    commands
        // 2d camera
        .spawn(CameraUiBundle::default())
        // texture
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: "Button".to_string(),
                    font: asset_server.load("asset_pack/FiraSans-Regular.ttf"),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        })
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
        .with(Player { x_rot: 0.0 })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(8.0, 9.0, 8.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(15.0, 15.0, 15.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            //rotation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        });

    let mut c = 0.0;
    for z in 0..CHUNK_SIZE {
        let mut b = 0.0;
        for y in 0..CHUNK_SIZE {
            let mut a = 0.0;
            for x in 0..CHUNK_SIZE {
                commands
                    // cube
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(c, b, a).into()),
                        transform: Transform::from_translation(Vec3::new(
                            x as f32, y as f32, z as f32,
                        )),
                        ..Default::default()
                    });

                a += 0.125;
            }
            b += 0.125;
        }
        c += 0.125;
    }
}
