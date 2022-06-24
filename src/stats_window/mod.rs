use bevy::{log, prelude::*};
use bevy_rapier2d::{plugin::RapierContext, prelude::InteractionGroups};

use crate::{camera::MainCamera, chimeras::ChimeraComponent};

mod ui;

pub struct StatsWindowPlugin;

impl Plugin for StatsWindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StatsWindow {
            target: None,
            cursor: None,
            target_setup: false,
            opened: false,
        })
        .add_startup_system(ui::setup_ui)
        .add_system(ui::update_window_stats)
        .add_system(ui::display_stats_window)
        .add_system(entity_click_detection)
        .add_system(setup_stats_target);
    }
}

#[derive(Component)]
pub struct StatsWindow {
    pub target: Option<Entity>,
    pub cursor: Option<Entity>,
    pub target_setup: bool,
    pub opened: bool,
}

#[derive(Component)]
pub struct StatsWindowTarget;

fn setup_stats_target(
    mut commands: Commands,
    mut stats_window: ResMut<StatsWindow>,
    // q_transform: Query<&Transform, With<ChimeraComponent>>,
    asset_server: Res<AssetServer>,
) {
    if stats_window.target_setup {
        return;
    }

    if let Some(target_entity) = stats_window.target {
        // get the corresponding transform
        // let transform = q_transform.get(target_entity).unwrap();

        commands.entity(target_entity).with_children(|parent| {
            let child_id = parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("target.png"),
                    ..default()
                })
                .insert(StatsWindowTarget)
                .id();

            stats_window.cursor = Some(child_id);
        });

        stats_window.target_setup = true;
    }
}

fn entity_click_detection(
    rapier_context: Res<RapierContext>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_chimera: Query<&ChimeraComponent>,
    mouse_button: Res<Input<MouseButton>>,
    mut stats_window: ResMut<StatsWindow>,
    mut commands: Commands,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        // detroy the previous target
        if let Some(target_entity) = stats_window.cursor {
            commands.entity(target_entity).despawn();
            stats_window.target = None;
            stats_window.cursor = None;
        }

        // get the camera
        let (camera, camera_gl_transform) = q_camera.single();

        // get the current window
        let curr_window = windows.get_primary().unwrap();

        // get the mouse position
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
            Vec2::ZERO
        };

        // get intersection, try to catch the chimera
        rapier_context.intersections_with_point(
            cursor_pos,
            InteractionGroups::all(),
            None,
            |entity| {
                // get the chimera component
                if let Ok(chi_compo) = q_chimera.get(entity) {
                    let stats = chi_compo.stats;
                    stats_window.target = Some(entity);
                    stats_window.target_setup = false;
                }
                // Return `false` instead if we want to stop searching for other colliders containing this point.
                true
            },
        );
    }
}
