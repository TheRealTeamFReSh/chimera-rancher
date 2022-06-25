// fire off a projectile on mouse click
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{camera::MainCamera, player::Player, projectile::Projectile};

use super::SpellKind;

pub fn fire_projectile_system(
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut player_q: Query<(&Transform, &mut Player)>,
    mouse_button: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    if let Some((player_transform, player)) = player_q.iter_mut().next() {
        let curr_window = windows.get_primary().unwrap();
        if mouse_button.just_pressed(MouseButton::Left)
            && matches!(player.active_spell, SpellKind::FireProjectile)
            && curr_window.cursor_position().unwrap().y > 75.0
            && player.can_fire_projetile
        {
            // TODO: retrieving cursor position should be refactored to a util.
            // Copied from `entity_click_detection`
            let (camera, camera_gl_transform) = camera.single();
            let cursor_pos = if let Some(screen_pos) = curr_window.cursor_position() {
                let window_size =
                    Vec2::new(curr_window.width() as f32, curr_window.height() as f32);
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
            let player_pos = player_transform.translation;
            let start_x = player_pos.x;
            let start_y = player_pos.y;
            let origin = Vec2::new(start_x, start_y);

            let direction_vector = (cursor_pos - origin).normalize();

            // spawn projectile
            let texture_handle = asset_server.load("small_triangle.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 1, 1);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let projectile = Projectile {
                despawn_timer: Timer::from_seconds(0.7, false),
                damage: 10.0,
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
                    linvel: Vec2::new(direction_vector.x * 600.0, direction_vector.y * 600.0),
                    angvel: 0.5,
                })
                .insert(Transform::from_translation(Vec3::new(
                    start_x + (direction_vector.x * 30.0),
                    start_y + (direction_vector.y * 30.0),
                    100.0,
                )))
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(7.0, 7.0))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(projectile);

            player.can_fire_projetile = false;
        } else {
            player.fire_projetile_cooldown.tick(time.delta());
            if player.fire_projetile_cooldown.just_finished() {
                player.can_fire_projetile = true;
            }
        }
    }
}
