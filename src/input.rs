use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::CursorMoved,
};
/// hi
pub fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    /// This system prints 'A' key state
    {
        if keyboard_input.pressed(KeyCode::A) {
            //println!("'A' currently pressed");
        }

        if keyboard_input.just_pressed(KeyCode::A) {
            println!("'A' just pressed");
        }

        if keyboard_input.just_released(KeyCode::A) {
            println!("'A' just released");
        }
    }
}

/// This system toggles the cursor's visibility when the escape key is pressed
pub fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

#[derive(Default)]
pub struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

pub fn print_mouse_events_system(
    mut state: Local<State>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
) {
    for _event in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {}

    for _event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {}

    for _event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {}

    for _event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {}
}
