use crate::startup::addon_folder_exists;
use rhai::{Engine, ImmutableString, RegisterFn};
use std::fmt::{Debug, Display};
#[derive(Debug)]
struct Addon {
    addon_id: String,
    enabled: bool,
    description: String,
}

pub fn load_addons() {
    // get the addon path
    // validate it then continue
    if addon_folder_exists() {}
    let mut engine = Engine::new();

    engine.register_fn("log", addon_log::<ImmutableString>);
    engine.register_fn("register_addon", register_addon::<ImmutableString>);
    engine.register_fn("finish", exit_mod);
    for _x in 0..0 {
        // Replace with addon iterator
        let result = engine.eval_file::<()>("addons/hello_world.mag".into());
        match result {
            Err(e) => {
                println!("Error: {:?}", e);
            }
            Ok(()) => {}
        }
    }
}

// Log to individual file!
fn addon_log<ImmutableString: Debug>(x: ImmutableString) {
    println!("put up a good show: {:?}!", x)
}

fn register_addon<ImmutableString: Display + Debug>(
    addon_id: ImmutableString,
    enabled: bool,
    description: ImmutableString,
) {
    if enabled {
        // insert addon into addon list
        let addon = Addon {
            addon_id: addon_id.to_string(),
            //    authors: ,
            enabled: true,
            description: description.to_string(),
        };
        println!("{:?}", addon);
    } else {
        println!("Addon {} Disabled", addon_id);
    }
}

fn exit_mod() {}
