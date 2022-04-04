use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;
use rand::prelude::*;

mod texture;

#[derive(Component)]
struct Player {
    moveable_up: bool,
    moveable_down: bool,
    moveable_left: bool,
    moveable_right: bool,
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Walls;

#[derive(Component)]
struct Soul {
    visible: bool,
}

const PLAYER_SPEED: f32 = 2.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Souler Power".to_string(),

            ..Default::default()
        })
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_system(movement)
        .add_system(animation)
        .add_system(follow_player)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(texture::set_texture_filters_to_nearest)
        .add_system(collect)
        .add_plugins(DefaultPlugins)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.4;
    commands.spawn_bundle(camera);
    let background_image: Handle<Image> = asset_server.load("floor.png");
    let music: Handle<AudioSource> = asset_server.load("music.ogg");
    let walls = asset_server.load("walls.png");
    commands
        .spawn_bundle(SpriteBundle {
            texture: background_image.into(),
            ..Default::default()
        })
        .insert(Background)
        .insert(Timer::from_seconds(65.0, true));
    audio.play(music);
    commands
        .spawn_bundle(SpriteBundle {
            texture: walls.into(),
            ..Default::default()
        })
        .insert(Walls)
        .insert(Timer::from_seconds(60.0, true));
}
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 24.0), 8, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(4., 4., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform::from_xyz(0.0, 0.0, 1.0))
        .insert(Player {
            moveable_up: true,
            moveable_down: true,
            moveable_left: true,
            moveable_right: true,
        })
        .insert(Timer::from_seconds(0.3, true));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("soul.png"),
                Vec2::new(16.0, 16.0),
                11,
                1,
            )),
            transform: Transform {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Soul { visible: true })
        .insert(Timer::from_seconds(0.3, true));
}

fn animation(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut soul_query: Query<
        (&mut Timer, &mut TextureAtlasSprite),
        (With<Soul>, Without<Player>, Without<Background>),
    >,
) {
    for (mut soul_timer, mut soul_sprite) in soul_query.iter_mut() {
        soul_timer.tick(time.delta());
        if soul_timer.finished() {
            if soul_sprite.index == 10 {
                soul_sprite.index = 0;
            } else {
                soul_sprite.index += 1;
            }
        }
    }

    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            if keyboard_input.pressed(KeyCode::W) {
                if sprite.index == 6 {
                    sprite.index = 7;
                    continue;
                } else if sprite.index == 7 {
                    sprite.index = 6;
                    continue;
                } else {
                    sprite.index = 6;
                    continue;
                }
            }
            if keyboard_input.pressed(KeyCode::S) {
                if sprite.index == 0 {
                    sprite.index = 1;
                    continue;
                } else if sprite.index == 1 {
                    sprite.index = 0;
                    continue;
                } else {
                    sprite.index = 0;
                    continue;
                }
            }
            if keyboard_input.pressed(KeyCode::A) {
                if sprite.index == 4 {
                    sprite.index = 5;
                    continue;
                } else if sprite.index == 5 {
                    sprite.index = 4;
                    continue;
                } else {
                    sprite.index = 4;
                    continue;
                }
            }
            if keyboard_input.pressed(KeyCode::D) {
                if sprite.index == 2 {
                    sprite.index = 3;
                    continue;
                } else if sprite.index == 3 {
                    sprite.index = 2;
                    continue;
                } else {
                    sprite.index = 2;
                    continue;
                }
            }
            if !keyboard_input.pressed(KeyCode::W)
                || !keyboard_input.pressed(KeyCode::A)
                || !keyboard_input.pressed(KeyCode::S)
                || !keyboard_input.pressed(KeyCode::D)
            {
                if sprite.index == 0 {
                    sprite.index = 1;
                    continue;
                } else if sprite.index == 1 {
                    sprite.index = 0;
                    continue;
                } else {
                    sprite.index = 0;
                    continue;
                }
            }
        }
    }
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Transform, With<Player>>,
    player_query: Query<&Player>,
    soul_query: Query<&Soul>,
    mut soul_entity: Query<&mut Visibility, With<Soul>>,
) {
    let mut soul_visibility = soul_entity.single_mut();
    let mut transform = player_positions.single_mut();
    let player = player_query.single();
    let soul = soul_query.single();
    if soul.visible {
        soul_visibility.is_visible = true;
    } else {
        soul_visibility.is_visible = false;
    }
    if keyboard_input.pressed(KeyCode::A) && player.moveable_left {
        println!("({}, {})", transform.translation.x, transform.translation.y);
        transform.translation.x -= PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::D) && player.moveable_right {
        println!("({}, {})", transform.translation.x, transform.translation.y);
        transform.translation.x += PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::S) && player.moveable_down {
        println!("({}, {})", transform.translation.x, transform.translation.y);
        transform.translation.y -= PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::W) && player.moveable_up {
        println!("({}, {})", transform.translation.x, transform.translation.y);
        transform.translation.y += PLAYER_SPEED;
    }
}

fn follow_player(
    player_positions: Query<&Transform, With<Player>>,
    mut camera_positions: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    windows: Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
) {
    let window = windows.get_primary().unwrap();
    let mut camera = camera_positions.single_mut();
    let player = player_positions.single();
    let mut player_entity = player_query.iter_mut().next().unwrap();
    /*if player.translation.x > 650.0 {
        player_entity.moveable_right = false;
    } else {
        player_entity.moveable_right = true;
    }
    if player.translation.x < -650.0 {
        player_entity.moveable_left = false;
    } else {
        player_entity.moveable_left = true;
    }
    if player.translation.y > 370.0 {
        player_entity.moveable_up = false;
    } else {
        player_entity.moveable_up = true;
    }
    if player.translation.y < -370.0 {
        player_entity.moveable_down = false;
    } else {
        player_entity.moveable_down = true;
    }*/

    if (keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::S))
        && (distance_to(player.translation.truncate(), camera.translation.truncate())
            > window.height() / 2.0 * 0.4 - 10.0)
    {
        camera.translation = player.translation;
    }
    if (keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::D))
        && (distance_to(player.translation.truncate(), camera.translation.truncate())
            > window.width() / 2.0 * 0.4 - 10.0)
    {
        camera.translation = player.translation;
    }
}

fn collect(
    mut timer_query: Query<&mut Timer, With<Background>>,
    time: Res<Time>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut soul_positions: Query<&mut Transform, With<Soul>>,
    mut soul_spawn_timer: Query<&mut Timer, (With<Walls>, Without<Player>, Without<Background>)>,
    player_positions: Query<&Transform, (With<Player>, Without<Soul>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let player = player_positions.single();
    let mut rng = thread_rng();
    let mut spawn_timer = soul_spawn_timer.single_mut();
    let mut soul = soul_positions.single_mut();
    let soul_locations: [Vec2; 10] = [
        Vec2::new(-13.0, -83.0),
        Vec2::new(583.0, -186.0),
        Vec2::new(416.0, 323.0),
        Vec2::new(586.0, -49.0),
        Vec2::new(-532.0, -30.0),
        Vec2::new(364.0, -148.0),
        Vec2::new(262.0, 29.0),
        Vec2::new(-422.0, 274.0),
        Vec2::new(-26.0, -141.0),
        Vec2::new(-86.0, 297.0),
    ];
    spawn_timer.tick(time.delta());
    let mut timer = timer_query.single_mut();
    timer.tick(time.delta());
    let music = asset_server.load("music.ogg");
    if timer.finished() {
        audio.play(music);
    }
    if spawn_timer.finished() {
        soul.translation = soul_locations[rng.gen_range(0..10)].extend(0.0);
    }
    if collide(
        player.translation,
        Vec2::new(16.0, 16.0),
        soul.translation,
        Vec2::new(16.0, 16.0),
    )
    .is_some()
        && keyboard_input.pressed(KeyCode::Space)
    {
        println!("you got a soul!");
        soul.translation = soul_locations[rng.gen_range(0..10)].extend(0.0);
    }
}

fn distance_to(point1: Vec2, point2: Vec2) -> f32 {
    let squared: f32 = ((point1.x - point2.x) * (point1.x - point2.x))
        + ((point1.y - point2.y) * (point1.y - point2.y));
    return squared.sqrt();
}
