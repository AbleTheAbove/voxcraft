use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::pass::ClearColor, ui::Val::Px,
};
mod addons;
mod config;
mod world;
use world::chunk::{Block, Chunk, ChunkOffset, Direction, CHUNKSIZE};
mod input;
mod player;
mod ui;

pub const ROOT_PATH: &str = "assets";
pub const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let game_meta_bar = format!("Voxcraft ver{}", GAME_VERSION);
    let config = config::load_config();

    addons::load_addons();

    // NOTE(Able): Prerendering work on chunks
    let location = ChunkOffset { x: 1, y: 1, z: 1 };
    let block = Block {
        id: 1,
        facing: Direction::Up,
    };
    let data = [[[block; CHUNKSIZE]; CHUNKSIZE]; CHUNKSIZE];
    let chunk = Chunk {
        location: location,
        data: data,
    };
    let world: Vec<Chunk> = vec![chunk];

    world[0];

    let player = player::Player {
        stats: player::Stats {
            speed: 1,
            strength: 1,
        },
        x_rot: 0.1,
    };

    App::build()
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
        .add_resource(player)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(input::toggle_cursor.system())
        .add_system(input::keyboard_input_system.system())
        .init_resource::<ui::ButtonMaterials>() // NOTE(Able): Here because game panics else
        .add_system(ui::text_update_system.system())
        .add_system(ui::button_system.system())
        .add_system(input::print_mouse_events_system.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    audio: Res<Audio>,
    button_materials: Res<ui::ButtonMaterials>,
) {
    let config = config::load_config();
    let font_size = (14 * config.ui_scale) as f32; // NOTE(Able): This should be used as the font size but it is not

    let music = asset_server.load("asset_pack/steps.mp3"); // TODO(Able): Replace with a good foot step sound/s
    audio.play(music);
    // UI Rendering
    commands
        .spawn(CameraUiBundle::default())
        // texture
        .spawn(ButtonBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                size: Size::new(Px(40.0), Px(20.0)),
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
                        font_size: font_size,
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
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: asset_server.load("asset_pack/FiraSans-Regular.ttf"),
                style: TextStyle {
                    font_size: font_size,

                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(ui::FpsText);
    // 3d Rendering
    commands
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(
                0.0 + CHUNKSIZE as f32 / 2.0,
                2.0 + CHUNKSIZE as f32,
                0.0 + CHUNKSIZE as f32 / 2.0,
            )),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(
                8.0 + CHUNKSIZE as f32,
                2.0 + CHUNKSIZE as f32,
                8.0 + CHUNKSIZE as f32,
            )),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(
                15.0 + CHUNKSIZE as f32,
                15.0 + CHUNKSIZE as f32,
                15.0 + CHUNKSIZE as f32,
            ))
            .looking_at(Vec3::default(), Vec3::unit_y()),
            //rotation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        });
    let color_inc = 1.0 / CHUNKSIZE as f32;
    let mut c = 0.0;

    for z in 0..CHUNKSIZE {
        // Z
        let mut b = 0.0;
        for y in 0..CHUNKSIZE {
            let mut a = 0.0;
            for x in 0..CHUNKSIZE {
                commands
                    // cube
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(a, b, c).into()),
                        transform: Transform::from_translation(Vec3::new(
                            x as f32, y as f32, z as f32,
                        )),
                        ..Default::default()
                    });

                a += color_inc;
            }
            b += color_inc;
        }
        c += color_inc;
    } // Z end
}
