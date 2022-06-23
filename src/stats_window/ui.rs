use bevy::{log, prelude::*};
use bevy_tweening::{lens::UiPositionLens, Animator, EaseFunction, Tween, TweeningType};

use crate::chimeras::ChimeraComponent;

use super::StatsWindow;

#[derive(Component)]
pub struct StatsWindowUI;

#[derive(Component)]
pub struct SpeedTextComponent;

#[derive(Component)]
pub struct AccelTextComponent;

#[derive(Component)]
pub struct DecelTextComponent;

const CLOSED_POS: Rect<Val> = Rect {
    right: Val::Px(-250.),
    top: Val::Px(0.),
    bottom: Val::Auto,
    left: Val::Auto,
};
const OPENED_POS: Rect<Val> = Rect {
    right: Val::Px(10.),
    top: Val::Px(0.),
    bottom: Val::Auto,
    left: Val::Auto,
};

pub fn update_window_stats(
    stats_window: Res<StatsWindow>,
    q_chimera: Query<&ChimeraComponent>,
    mut q_speed_text: Query<&mut Text, With<SpeedTextComponent>>,
    mut q_accel_text: Query<
        &mut Text,
        (
            With<AccelTextComponent>,
            Without<SpeedTextComponent>,
            Without<DecelTextComponent>,
        ),
    >,
    mut q_deccel_text: Query<
        &mut Text,
        (
            With<DecelTextComponent>,
            Without<SpeedTextComponent>,
            Without<AccelTextComponent>,
        ),
    >,
) {
    if let Some(target_entity) = stats_window.target {
        // get the stats
        let stats = q_chimera.get(target_entity).unwrap().stats;
        for mut text in q_speed_text.iter_mut() {
            text.sections[0].value = format!("Speed: {:.2}", stats.speed);
        }
        for mut text in q_accel_text.iter_mut() {
            text.sections[0].value = format!("Acceleration: {:.2}", stats.accel);
        }
        for mut text in q_deccel_text.iter_mut() {
            text.sections[0].value = format!("Deceleration: {:.2}", stats.decel);
        }
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let container = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.),
                left: Val::Px(0.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    };

    let border = NodeBundle {
        style: Style {
            position: CLOSED_POS,
            size: Size::new(Val::Px(200.0), Val::Px(250.0)),
            border: Rect::all(Val::Px(2.0)),
            ..default()
        },
        color: Color::rgb(0.65, 0.65, 0.65).into(),
        ..default()
    };

    let content_container = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::FlexStart,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::FlexEnd,
            ..default()
        },
        color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    };

    let content_text = TextBundle {
        style: Style {
            margin: Rect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
            },
            ..default()
        },
        text: Text::with_section(
            "Chimera stats",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..default()
    };

    let speed_text = TextBundle {
        style: Style {
            margin: Rect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
            },
            ..default()
        },
        text: Text::with_section(
            "Speed: 0.0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..default()
    };

    let accel_text = TextBundle {
        style: Style {
            margin: Rect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
            },
            ..default()
        },
        text: Text::with_section(
            "Acceleration: 0.0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..default()
    };

    let deccel_text = TextBundle {
        style: Style {
            margin: Rect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
            },
            ..default()
        },
        text: Text::with_section(
            "Deceleration: 0.0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..default()
    };

    // spawn the ui
    commands.spawn_bundle(container).with_children(|parent| {
        parent
            .spawn_bundle(border)
            .with_children(|parent| {
                parent
                    .spawn_bundle(content_container)
                    .with_children(|parent| {
                        parent.spawn_bundle(content_text);
                        parent.spawn_bundle(speed_text).insert(SpeedTextComponent);
                        parent.spawn_bundle(accel_text).insert(AccelTextComponent);
                        parent.spawn_bundle(deccel_text).insert(DecelTextComponent);
                    });
            })
            .insert(Animator::<Style>::default())
            .insert(StatsWindowUI);
    });
}

pub fn display_stats_window(
    mut stats_window: ResMut<StatsWindow>,
    mut q_anim: Query<&mut Animator<Style>, With<StatsWindowUI>>,
) {
    // get animator
    let mut animator = q_anim.get_single_mut().unwrap();

    // if finished
    if stats_window.target != None && !stats_window.opened {
        log::info!("opening");

        stats_window.opened = true;
        animator.set_tweenable(Tween::new(
            EaseFunction::CubicInOut,
            TweeningType::Once,
            std::time::Duration::from_millis(500),
            UiPositionLens {
                start: CLOSED_POS,
                end: OPENED_POS,
            },
        ));
    } else if stats_window.target == None && stats_window.opened {
        log::info!("closing");

        stats_window.opened = false;
        animator.set_tweenable(Tween::new(
            EaseFunction::CubicInOut,
            TweeningType::Once,
            std::time::Duration::from_millis(500),
            UiPositionLens {
                start: OPENED_POS,
                end: CLOSED_POS,
            },
        ));
    }
}
