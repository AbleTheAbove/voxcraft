use bevy::prelude::*;

/// This system prints 'A' key state
pub fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
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

/// This system toggles the cursor's visibility when the space bar is pressed
pub fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}
