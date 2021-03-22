// We will be trying to minimize code in main.rs
extern crate piston_window;
use piston_window::*;

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

    let mut window: PistonWindow = WindowSettings::new(
        game_meta_bar,
        [config.width as u32, config.height as u32], // u32 is required here
    )
    .exit_on_esc(true) // NOTE(Able): remove in production! This is just used for quick exits
    .build()
    .unwrap();

    let assets = root.join("assets");
    let mut glyphs = window
        // TODO(Able): Provide a texture pack loading option with a fall back to a built in texture pack
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap(); // TODO(Able): Handle errors here and remove the unwrap
    let ui_color = [1.0, 1.0, 1.0, 1.0];
    {
        // NOTE(Able): Prerendering work on chunks

        let chunk_data = [[[0; 32]; 32]; 32];
        let chunk = Chunk { data: chunk_data };
        chunk.fetch(0, 1, 2);
    }
    // IDEA(Able): Provide this as an option > More testing needed
    window.set_lazy(true); // NOTE(Able): Only updates when it needs too
    while let Some(e) = window.next() {
        window.draw_3d(&e, |_window| {
            let _args = e.render_args().unwrap(); // NOTE(Able):  arguments such as window size

            // TODO(Able): sort out how to do this
            // println!("{:?}", window);
        }); // Draw the 3D world first
        window.draw_3d(&e, |_window| {
            let _args = e.render_args().unwrap(); // NOTE(Able):  arguments such as window size
        }); // draw the particles here
        window.draw_2d(&e, |c, g, device| {
            // NOTE(Able): Clears to black with 100%
            clear([0.0, 0.0, 0.0, 0.0], g);
            // Startup UI
            {
                let startup_transform = c
                    .transform
                    .trans(1.0 * config.ui_scale as f64, 12.0 * config.ui_scale as f64); // TODO(Able): Tweak the ui scaling

                text::Text::new_color(ui_color, font_size as u32)
                    .draw(text, &mut glyphs, &c.draw_state, startup_transform, g)
                    .unwrap();
            }
            // Version UI
            {
                let version_transform = c
                    .transform
                    .trans(1.0 * config.ui_scale as f64, 24.0 * config.ui_scale as f64); // TODO(Able): Tweak the ui scaling
                let version_string = format!("Game version: {}", GAME_VERSION);
                text::Text::new_color(ui_color, font_size as u32)
                    .draw(
                        &version_string,
                        &mut glyphs,
                        &c.draw_state,
                        version_transform,
                        g,
                    )
                    .unwrap();
            }

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        }); // NOTE(Able): Draw the UI last
    }
}
