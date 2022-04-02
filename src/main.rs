use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod texture;

#[derive(Component)]
struct Player;

const PLAYER_SPEED: f32 = 5.0;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_system(movement)
        .add_system(animation)
        .add_system(follow_player)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    let texture_handle = asset_server.load("tiles.png");

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(16, 16),
            ChunkSize(8, 8),
            TileSize(16.0, 16.0),
            TextureSize(144.0, 144.0),
        ),
        0u16,
        0u16,
    );

    layer_builder.set_all(TileBundle::default());

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());

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
            let move_to = move_towards(
                transform.translation.truncate(),
                Vec2::new(
                    transform.translation.x - PLAYER_SPEED,
                    transform.translation.y,
                ),
                1.0,
            );
            transform.translation = move_to.extend(0.0);
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        }
        if keyboard_input.pressed(KeyCode::D) {
            let move_to = move_towards(
                transform.translation.truncate(),
                Vec2::new(
                    transform.translation.x + PLAYER_SPEED,
                    transform.translation.y,
                ),
                1.0,
            );
            transform.translation = move_to.extend(0.0);
            transform.rotation = Quat::default();
        }
        if keyboard_input.pressed(KeyCode::S) {
            let move_to = move_towards(
                transform.translation.truncate(),
                Vec2::new(
                    transform.translation.x,
                    transform.translation.y - PLAYER_SPEED,
                ),
                1.0,
            );
            transform.translation = move_to.extend(0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            let move_to = move_towards(
                transform.translation.truncate(),
                Vec2::new(
                    transform.translation.x,
                    transform.translation.y + PLAYER_SPEED,
                ),
                1.0,
            );
            transform.translation = move_to.extend(0.0);
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

fn move_towards(a: Vec2, b: Vec2, distance: f32) -> Vec2 {
    let vector = Vec2::new(b.x - a.x, b.y - a.y);
    let length: f32 = (vector.x * vector.x + vector.y * vector.y).sqrt();
    let unit_vector = Vec2::new(vector.x / length, vector.y / length);
    return Vec2::new(a.x + unit_vector.x * 2.0, a.y + unit_vector.y * distance);
}
