use bevy::{asset::embedded_asset, prelude::*};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, |app: &mut App| {
            embedded_asset!(app, "not_working");
            embedded_asset!(app, "../assets/working")
        }))
        .run();
}
