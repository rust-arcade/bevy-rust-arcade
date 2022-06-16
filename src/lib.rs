use bevy::prelude::*;

pub struct RustArcadePlugin;
impl Plugin for RustArcadePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gamepad_connections_system);
    }
}

pub struct ArcadeGamepad(pub Gamepad);

pub enum ArcadeInput {
    JoyUp,
    JoyDown,
    JoyLeft,
    JoyRight,
    JoyButton,
    ButtonTop1,
    ButtonTop2,
    ButtonTop3,
    ButtonTop4,
    ButtonTop5,
    ButtonTop6,
    ButtonLeftSide,
    ButtonRightSide,
    ButtonFront1,
    ButtonFront2,
}

fn gamepad_connections_system(
    mut commands: Commands,
    arcade_gamepad: Option<Res<ArcadeGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                // if we don't have any gamepad yet, use this one
                if arcade_gamepad.is_none() {
                    commands.insert_resource(ArcadeGamepad(*id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(ArcadeGamepad(old_id)) = arcade_gamepad.as_deref() {
                    if old_id == id {
                        commands.remove_resource::<ArcadeGamepad>();
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}
