use bevy::{prelude::*, render::camera::RenderTarget};

use crate::states::GameStates;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // cameras should always be there
        app.add_startup_system(setup_camera);

        // on update
        app.add_system_set(
            SystemSet::on_update(GameStates::Game).with_system(camera_movement_system),
        );
    }
}

#[derive(Component)]
pub struct MainCamera {
    smooth_speed: f32,
    target_to_mouse_percent: f32,
}

#[derive(Component)]
pub struct CameraTarget;

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera {
            smooth_speed: 0.1,
            target_to_mouse_percent: 0.1,
        });

    // let window = windows.get_primary().unwrap();
    // commands.spawn_bundle(UiCameraBundle {
    //     transform: Transform::from_xyz(0.0, -window.height() / 2., 1.0),
    //     ..default()
    // });
    commands.spawn_bundle(UiCameraBundle::default());
}

// the camera movement system grabs the position between the player
// and the mouse, then lerps the camera to this position
fn camera_movement_system(
    windows: Res<Windows>,
    mut q_camera: Query<(&Camera, &GlobalTransform, &mut Transform, &MainCamera)>,
    q_target: Query<&GlobalTransform, With<CameraTarget>>,
) {
    // get the camera
    let (camera, camera_gl_transform, mut camera_transform, camera_config) = q_camera.single_mut();

    for target_gl_transform in q_target.iter() {
        // get target coordinates
        let target_coords = Vec2::new(
            target_gl_transform.translation.x,
            target_gl_transform.translation.y,
        );

        // get the current window
        let curr_window = if let RenderTarget::Window(id) = camera.target {
            windows.get(id).unwrap()
        } else {
            windows.get_primary().unwrap()
        };

        // check if the cursor is inside the window and get its position
        // else set it to the player pos (no lerp between them)
        let cursor_pos = if let Some(screen_pos) = curr_window.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(curr_window.width() as f32, curr_window.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_gl_transform.compute_matrix() * camera.projection_matrix.inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            // use this value
            world_pos
        } else {
            target_coords
        };

        // set target position, 60% player, 40% cursor
        let camera_target_pos = Vec2::lerp(
            target_coords,
            cursor_pos,
            camera_config.target_to_mouse_percent,
        );
        // lerp position for smoothing
        let camera_new_pos = Vec2::lerp(
            Vec2::new(
                camera_transform.translation.x,
                camera_transform.translation.y,
            ),
            camera_target_pos,
            camera_config.smooth_speed,
        );

        // set camera position
        camera_transform.translation.x = camera_new_pos.x;
        camera_transform.translation.y = camera_new_pos.y;
    }
}
