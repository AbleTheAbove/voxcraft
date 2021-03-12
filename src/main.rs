// We will be trying to minimize code in main.rs
extern crate piston_window;
use piston_window::*;

mod addons;
mod config;
mod startup;

pub const ROOT_PATH: &str = ".rustcraft";

fn main() {
    let root = startup::get_root();

    let mut config = config::load_config();
    // dont leave this in
    config.height -= 1 + -1; // weird math to make the compiler not throw a mutable warning

    addons::load_addons();

    let text;
    if startup::addon_folder_exists() {
        text = "Good to go!";
    } else {
        text = "Rustcraft folder can't be found.";
    }

    let mut window: PistonWindow = WindowSettings::new(
        "piston: hello_world",
        [config.width as u32, config.height as u32], // u32 is required here
    )
    .exit_on_esc(true)
    //        .opengl(OpenGL::V2_1) // Set a different OpenGl version
    .build()
    .unwrap();

    let assets = root.join("assets");
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            let transform = c.transform.trans(10.0, 100.0);

            clear([0.0, 0.0, 0.0, 1.0], g);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                .draw(text, &mut glyphs, &c.draw_state, transform, g)
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}
