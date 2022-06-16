use bevy::prelude::*;
use rust_arcade_plugin::{ArcadeGamepad, RustArcadePlugin};

fn main() {
    // Define window
    let window = WindowDescriptor {
        title: "Inputs".to_string(),
        width: 1280.0,
        height: 720.0,
        resizable: false,
        ..Default::default()
    };

    App::new()
        .insert_resource(window)
        .add_plugins(DefaultPlugins)
        .add_plugin(RustArcadePlugin)
        .add_system(gamepad_input_events)
        .run();
}

fn gamepad_input_events(
    my_gamepad: Option<Res<ArcadeGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };

    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        if id.0 != gamepad.0 {
            // event not from our gamepad
            continue;
        }

        println!("{:?}", kind);

        /*
        use GamepadEventType::{AxisChanged, ButtonChanged};

        match kind {
            AxisChanged(GamepadAxisType::RightStickX, x) => {
                // Right Stick moved (X)
            }
            AxisChanged(GamepadAxisType::RightStickY, y) => {
                // Right Stick moved (Y)
            }
            ButtonChanged(GamepadButtonType::DPadDown, val) => {
                // buttons are also reported as analog, so use a threshold
                if *val > 0.5 {
                    // button pressed
                }
            }
            _ => {} // don't care about other inputs
        }
        */
    }
}
