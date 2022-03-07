use bevy::prelude::*;
/*
fn hello_world() {
    println!("Hello, world!");
}
*/

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("tile-map.png"),
        ..Default::default()
    });
}

fn main () {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        //.add_system(hello_world)
        .run()
}
