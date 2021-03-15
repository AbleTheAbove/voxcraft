// We will be trying to minimize code in main.rs
extern crate piston_window;
use piston_window::*;

mod addons;
mod config;
mod startup;

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
    //        .opengl(OpenGL::V2_1) // Set a different OpenGl version
    .build()
    .unwrap();

    let assets = root.join("assets");
    let mut glyphs = window
        // TODO(Able): Provide a texture pack loading option with a fall back to a built in texture pack
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap(); // TODO(Able): Handle errors here and remove the unwrap
    let ui_color = [1.0, 1.0, 1.0, 1.0];
    // IDEA(Able): Provide this as an option > More testing needed
    window.set_lazy(true); // NOTE(Able): Only updates when it needs too
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g); // NOTE(Able): Clears to black with 100%

            let transform = c
                .transform
                .trans(1.0 * config.ui_scale as f64, 12.0 * config.ui_scale as f64); // TODO(Able): Tweak the ui scaling

            text::Text::new_color(ui_color, font_size as u32)
                .draw(text, &mut glyphs, &c.draw_state, transform, g)
                .unwrap();

            let transform2 = c
                .transform
                .trans(1.0 * config.ui_scale as f64, 24.0 * config.ui_scale as f64); // TODO(Able): Tweak the ui scaling
            let version_string = format!("Game version: {}", GAME_VERSION);
            text::Text::new_color(ui_color, font_size as u32)
                .draw(&version_string, &mut glyphs, &c.draw_state, transform2, g)
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        }); // NOTE(Able): Draw the UI last
    }
}
