use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::ecs::{Query, Res, With};
use bevy::ui::widget::Text;

// NOTE(Able): The FPS text struct
pub struct FpsText;

pub fn text_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}\n", average);
            }
        }
    }
}
