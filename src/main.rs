use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_system(movement)
        .add_system(animation)
        .add_system(follow_player)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugins(DefaultPlugins)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(2., 2., 2.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Timer::from_seconds(0.1, true));
}

fn animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W)
            || keyboard_input.pressed(KeyCode::A)
            || keyboard_input.pressed(KeyCode::S)
            || keyboard_input.pressed(KeyCode::D)
        {
            timer.tick(time.delta());
        } else {
            sprite.index = 0;
        }
        if timer.finished() {
            let texture_atlas = texture_atlases.get(&*texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Transform, With<Player>>,
) {
    for mut transform in player_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 2.;
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 2.;
            transform.rotation = Quat::default();
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 2.;
        }
    }
}

fn follow_player(
    player_positions: Query<&Transform, With<Player>>,
    mut camera_positions: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let mut camera = camera_positions.single_mut();
    let player = player_positions.single();
    if distance_to(camera.translation.truncate(), player.translation.truncate()) > 100.0 {
        camera.translation = player.translation;
    }
}

fn distance_to(point1: Vec2, point2: Vec2) -> f32 {
    let squared: f32 = ((point1.x - point2.x) * (point1.x - point2.x))
        + ((point1.y - point2.y) * (point1.y - point2.y));
    return squared.sqrt();
}
