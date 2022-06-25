use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animals::{AnimalAttributesResource, AnimalComponent},
    camera::CameraTarget,
    camera::MainCamera,
    chimeras::{ChimeraPartAttributes, ChimeraPartKind},
    constants,
    projectile::Projectile,
    states::GameStates,
};

#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
    pub acceleration: f32,
    pub friction: f32,
    pub capture_distance: f32,
    pub inventory: PlayerInventory,
}

#[derive(Debug)]
pub struct PlayerInventory {
    pub chimera_parts: Vec<ChimeraPartAttributes>,
}

#[derive(Component)]
struct AnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // on enter
        app.add_system_set(SystemSet::on_enter(GameStates::Game).with_system(spawn_player));

        // on update
        app.add_system_set(
            SystemSet::on_update(GameStates::Game)
                .with_system(animate_player)
                .with_system(move_player)
                .with_system(capture_animal)
                .with_system(fire_projectile),
        );
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
        speed: 300.,
        acceleration: 0.1,
        friction: 0.2,
        capture_distance: 200.0,
        inventory: PlayerInventory {
            chimera_parts: Vec::new(),
        },
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
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(CameraTarget);
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Velocity,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, velocity) in query.iter_mut() {
        // if the player moves, update the animation
        if velocity.linvel.length() > 20. {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        } else {
            // else set it to the first frame (better will be to set it to the idle
            // animation)
            timer.0.reset();
            sprite.index = 0;
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Velocity, &mut TextureAtlasSprite)>,
) {
    for (player, mut vel, mut sprite) in query.iter_mut() {
        let mut input_direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            input_direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            input_direction.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            input_direction.y += 1.0;
        } else if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            input_direction.y -= 1.0;
        }

        // if the player didn't move, use friction
        if input_direction == Vec2::ZERO {
            vel.linvel = Vec2::lerp(vel.linvel, Vec2::ZERO, player.friction);
        } else {
            // normalize in order to have a maximum speed of 1 (dir.length == 1)
            let dir_vel = input_direction.normalize() * player.speed;
            vel.linvel = Vec2::lerp(vel.linvel, dir_vel, player.acceleration);

            // flip sprite depending on the direction
            sprite.flip_x = dir_vel.x < 0.0;
        }
    }
}

// capture and animal by pressing e near it
fn capture_animal(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player)>,
    animal_query: Query<(&Transform, &AnimalComponent, Entity)>,
    animal_attr_res: Res<AnimalAttributesResource>,
) {
    let capture_input = keyboard_input.just_pressed(KeyCode::E);

    if capture_input {
        for (player_transform, mut player) in player_query.iter_mut() {
            let player_pos = player_transform.translation;

            for (animal_transform, animal, animal_entity) in animal_query.iter() {
                let animal_pos = animal_transform.translation;

                // if player is in range of the animal, collect the animal
                if player_pos.distance(animal_pos) < player.capture_distance {
                    // get the chimera part attributes from the animal component
                    let animal_stats = animal.stats;
                    let animal_attr = &animal_attr_res[&animal_stats.kind];
                    let chimera_attr_head = ChimeraPartAttributes {
                        attack: animal_stats.attack * constants::HEAD_ATTACK_PERCENT,
                        health: animal_stats.health * constants::HEAD_HEALTH_PERCENT,
                        speed: animal_stats.speed * constants::HEAD_SPEED_PERCENT,
                        accel: animal_stats.accel * constants::HEAD_ACCEL_PERCENT,
                        decel: animal_stats.decel * constants::HEAD_DECEL_PERCENT,
                        collider_size: animal_attr.collider_size,
                        texture: animal_attr.head_texture.clone(),
                        kind: ChimeraPartKind::Head(animal_stats.kind),
                    };
                    let chimera_attr_tail = ChimeraPartAttributes {
                        attack: animal_stats.attack * constants::TAIL_ATTACK_PERCENT,
                        health: animal_stats.health * constants::TAIL_HEALTH_PERCENT,
                        speed: animal_stats.speed * constants::TAIL_SPEED_PERCENT,
                        accel: animal_stats.accel * constants::TAIL_ACCEL_PERCENT,
                        decel: animal_stats.decel * constants::TAIL_DECEL_PERCENT,
                        collider_size: animal_attr.collider_size,
                        texture: animal_attr.tail_texture.clone(),
                        kind: ChimeraPartKind::Tail(animal_stats.kind),
                    };

                    // add chimera parts to inventory
                    player.inventory.chimera_parts.push(chimera_attr_head);
                    player.inventory.chimera_parts.push(chimera_attr_tail);

                    // print the attributes from the parts
                    //println!("capturing {:?}", animal.stats);
                    //println!("inventory {:?}", player.inventory);

                    // despawn the animal
                    commands.entity(animal_entity).despawn_recursive();
                    break;
                }
            }
        }
    }
}

// fire off a projectile on mouse click
fn fire_projectile(
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    player_q: Query<(&Transform, &mut Player)>,
    mouse_button: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        // TODO: retrieving cursor position should be refactored to a util.
        // Copied from `entity_click_detection`
        let (camera, camera_gl_transform) = camera.single();
        let curr_window = windows.get_primary().unwrap();
        let cursor_pos = if let Some(screen_pos) = curr_window.cursor_position() {
            let window_size = Vec2::new(curr_window.width() as f32, curr_window.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world =
                camera_gl_transform.compute_matrix() * camera.projection_matrix.inverse();
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            let world_pos: Vec2 = world_pos.truncate();

            world_pos
        } else {
            Vec2::ZERO
        };

        // get player position
        for (player_transform, _) in player_q.iter() {
            let player_pos = player_transform.translation;
            let start_x = player_pos.x;
            let start_y = player_pos.y;
            let origin = Vec2::new(start_x, start_y);

            // spawn projectile
            let texture_handle = asset_server.load("small_triangle.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 1, 1);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let projectile = Projectile {
                accel: 200.0,
                target: Vec2::new(cursor_pos.x, cursor_pos.y),
                speed: 500.,
            };
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        translation: Vec3::new(origin.x, origin.y, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Velocity {
                    linvel: Vec2::new(0.0, 0.0),
                    angvel: 0.5,
                })
                .insert(Transform::from_translation(Vec3::new(
                    start_x, start_y, 100.0,
                )))
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(24.0, 24.0))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(projectile);
        }
    }
}
