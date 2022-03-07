use bevy::prelude::*;
/*
fn hello_world() {
    println!("Hello, world!");
}
*/

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, 
) {
    let texture_handle = asset_server.load("tile-map.png");
    // sprites are 16x16, in a 3x3 grid
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 3, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(3.0)),
        // spawn the 5th sprite (the player).
        sprite: TextureAtlasSprite::new(5),
        //texture: asset_server.load("tile-map.png"),
        ..Default::default()
    });
}

fn main () {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        //.add_system(hello_world)
        .run()
}
