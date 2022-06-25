use std::{f32::consts::PI, time::Duration};

use bevy::{log, prelude::*};

use crate::{constants, states::GameStates};

pub struct DayCyclePlugin;

impl Plugin for DayCyclePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DayCycleResource {
            day_timer: Timer::from_seconds(constants::DAY_LENGTH, true),
            day_length: constants::DAY_LENGTH,
            max_alpha: constants::MAX_ALPHA,
            min_alpha: constants::MIN_ALPHA,
            night_hours: constants::NIGHT_HOURS,
            days_passed: 0,
        });

        // on enter
        app.add_system_set(
            SystemSet::on_enter(GameStates::Game).with_system(setup_lighting_system),
        );

        // on update
        app.add_system_set(SystemSet::on_update(GameStates::Game).with_system(day_cycle_system));
    }
}

pub struct DayCycleResource {
    pub day_timer: Timer,
    pub day_length: f32,
    pub max_alpha: f32,
    pub min_alpha: f32,
    pub night_hours: f32,
    pub days_passed: u8,
}

impl DayCycleResource {
    pub fn get_alpha(&self) -> f32 {
        let x = self.day_timer.elapsed_secs();
        let fx = f32::max(f32::min(f32::cos(x * 2. * PI / 24. - 0.5) * 1.8, 1.), 0.);

        // linear regression
        fx * (self.max_alpha - self.min_alpha) + self.min_alpha
    }

    pub fn get_hour(&self) -> u8 {
        ((self.day_timer.elapsed_secs() / self.day_length) * 24.0) as u8
    }

    pub fn get_minute(&self) -> u8 {
        ((((self.day_timer.elapsed_secs() / self.day_length) * 24.0) - (self.get_hour() as f32))
            * 60.0) as u8
    }
}

fn day_cycle_system(
    mut lighting_query: Query<&mut UiColor, With<LightingComponent>>,
    mut day_cycle_resource: ResMut<DayCycleResource>,
    time: Res<Time>,
) {
    day_cycle_resource.day_timer.tick(time.delta());
    if day_cycle_resource.day_timer.just_finished() {
        day_cycle_resource.days_passed += 1;
    }
    for mut uicolor in lighting_query.iter_mut() {
        log::trace!(
            "day {} hour: {} minute: {} => alpha: {}",
            day_cycle_resource.days_passed,
            day_cycle_resource.get_hour(),
            day_cycle_resource.get_minute(),
            day_cycle_resource.get_alpha()
        );
        *uicolor = UiColor(Color::rgba(1., 1., 1., day_cycle_resource.get_alpha()));
    }
}

#[derive(Component)]
pub struct LightingComponent;

fn setup_lighting_system(
    mut commands: Commands,
    mut day_cycle_resource: ResMut<DayCycleResource>,
    asset_server: Res<AssetServer>,
) {
    day_cycle_resource
        .day_timer
        .set_elapsed(Duration::from_secs_f32(
            (constants::STARTING_HOUR / 24.0) * constants::DAY_LENGTH,
        ));

    commands
        .spawn()
        .insert(LightingComponent)
        .insert_bundle(ImageBundle {
            transform: Transform::from_xyz(0., 0., constants::Z_DAY_CYCLE),
            image: asset_server.load("lighting.png").into(),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect::all(Val::Px(0.0)),
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..default()
            },
            color: Color::rgba(1., 1., 1., day_cycle_resource.get_alpha()).into(),
            ..default()
        })
        .insert(Name::new("Lighting"));
}
