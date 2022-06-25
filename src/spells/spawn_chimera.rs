use bevy::prelude::*;

use crate::camera::MainCamera;

pub fn spawn_chimera_system(windows: Res<Windows>, camera_query: Query<MainCamera>) {
    let curr_window = windows.get_primary();
}
