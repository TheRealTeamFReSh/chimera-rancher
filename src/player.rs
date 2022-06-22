use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
    pub acceleration: f32,
}

#[derive(Component)]
struct AnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(animate_player)
            .add_system(move_player)
            .add_system(follow_player_camera);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("mage.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(77.0, 50.0), 8, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player = Player {
        speed: 5.5,
        acceleration: 1.0,
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 12.0, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(Transform::from_translation(Vec3::new(0.0, 50.0, 100.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(25.0, 10.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(player)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Velocity, &mut TextureAtlasSprite)>,
) {
    for (player, mut vel, mut sprite) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            vel.linvel.x -= player.speed * 1.0;
            sprite.flip_x = true;
        } else if keyboard_input.pressed(KeyCode::D) {
            vel.linvel.x += player.speed * 1.0;
            sprite.flip_x = false;
        } else if keyboard_input.pressed(KeyCode::W) {
            vel.linvel.y += player.speed * 1.0;
        } else if keyboard_input.pressed(KeyCode::S) {
            vel.linvel.y -= player.speed * 1.0;
        } else {
            vel.linvel.x = 0.0;
            vel.linvel.y = 0.0;
        }
    }
}

fn follow_player_camera(
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Some(player) = player.iter().next() {
        for mut transform in camera.iter_mut() {
            transform.translation.x = player.translation.x;
        }
    }
}
