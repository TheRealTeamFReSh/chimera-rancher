use bevy::{log, prelude::*};
use bevy_tweening::{lens::UiPositionLens, Animator, EaseFunction, Tween, TweeningType};

use crate::{chimeras::ChimeraComponent, constants::MaxStats, health::Health};

use super::StatsWindow;

#[derive(Component)]
pub struct StatsWindowUI;

#[derive(Component)]
pub struct SpeedBarComponent;

#[derive(Component)]
pub struct AccelBarComponent;

#[derive(Component)]
pub struct DecelBarComponent;

const CLOSED_POS: Rect<Val> = Rect {
    right: Val::Px(-350.),
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
    // mut commands: Commands,
    stats_window: Res<StatsWindow>,
    maxi_stats: Res<MaxStats>,
    q_chimera: Query<(&Health, &ChimeraComponent)>,
    q_ui_bar: Query<(&Children, &UIBar), (Without<MaxBarComponent>, Without<ValueBarComponent>)>,
    mut q_ui_bar_max: Query<
        (&Children, &mut Style),
        (With<MaxBarComponent>, Without<ValueBarComponent>),
    >,
    mut q_ui_bar_value: Query<&mut Style, With<ValueBarComponent>>,
) {
    if let Some(target_entity) = stats_window.target {
        // get the stats
        let (health, chimera) = q_chimera.get(target_entity).unwrap();
        let stats = chimera.stats;

        // for each bar
        for (children, bar) in q_ui_bar.iter() {
            // set value according to bartype
            let (max_value_possible, max_value, value) = match bar.bartype {
                BarStatType::Acceleration => (maxi_stats.accel, stats.accel, stats.accel),
                BarStatType::Deceleration => (maxi_stats.decel, stats.decel, stats.decel),
                BarStatType::Speed => (maxi_stats.speed, stats.speed, stats.speed),
                BarStatType::Attack => (maxi_stats.attack, stats.attack, stats.attack),
                BarStatType::Health => (maxi_stats.health, health.max_health, health.health * 0.8),
            };

            // getting max_value
            for child in children.iter() {
                if let Ok((children2, mut style)) = q_ui_bar_max.get_mut(*child) {
                    style.size = Size::new(
                        Val::Percent(100. * max_value / max_value_possible),
                        Val::Percent(100.),
                    );

                    // getting value
                    for child2 in children2.iter() {
                        if let Ok(mut style) = q_ui_bar_value.get_mut(*child2) {
                            style.size = Size::new(
                                Val::Percent(100. * value / max_value),
                                Val::Percent(100.),
                            );
                        }
                    }
                }
            }
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
            size: Size::new(Val::Px(300.0), Val::Px(450.0)),
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
            align_items: AlignItems::Center,
            overflow: Overflow::Hidden,
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

    // spawn the ui
    commands.spawn_bundle(container).with_children(|parent| {
        parent
            .spawn_bundle(border)
            .with_children(|parent| {
                parent
                    .spawn_bundle(content_container)
                    .with_children(|parent| {
                        parent.spawn_bundle(content_text);
                        // speed
                        parent.spawn_bundle(create_stat_text(&asset_server, "Speed"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Speed));
                        // accel
                        parent.spawn_bundle(create_stat_text(&asset_server, "Acceleration"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Acceleration));
                        // decel
                        parent.spawn_bundle(create_stat_text(&asset_server, "Deceleration"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Deceleration));
                        // attack
                        parent.spawn_bundle(create_stat_text(&asset_server, "Attack"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Attack));
                        // decel
                        parent.spawn_bundle(create_stat_text(&asset_server, "Health"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Health));
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

#[derive(Debug)]
pub enum BarStatType {
    Acceleration,
    Deceleration,
    Speed,
    Attack,
    Health,
}

#[derive(Component)]
pub struct UIBar {
    max_possible_value: f32,
    max_value: f32,
    value: f32,
    bartype: BarStatType,
}

impl UIBar {
    fn from_type(bartype: BarStatType) -> Self {
        UIBar {
            max_possible_value: 100.,
            max_value: 100.,
            value: 50.,
            bartype,
        }
    }
}

#[derive(Component)]
pub struct MaxBarComponent;

#[derive(Component)]
pub struct ValueBarComponent;

pub fn create_ui_bar(parent: &mut ChildBuilder, bar: UIBar) {
    let full_bar = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(80.), Val::Px(15.)),
            margin: Rect {
                bottom: Val::Px(15.),
                top: Val::Auto,
                left: Val::Auto,
                right: Val::Auto,
            },
            ..default()
        },
        color: Color::rgb_u8(10, 10, 10).into(),
        ..default()
    };

    let max_health_bar = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Percent(100. * bar.max_value / bar.max_possible_value),
                Val::Percent(100.),
            ),
            ..default()
        },
        color: Color::rgb_u8(69, 69, 69).into(),
        ..default()
    };

    let health_bar = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Percent(100. * bar.value / bar.max_value),
                Val::Percent(100.),
            ),
            ..default()
        },
        color: Color::rgb_u8(15, 117, 37).into(),
        ..default()
    };

    parent
        .spawn_bundle(full_bar)
        .with_children(|parent| {
            parent
                .spawn_bundle(max_health_bar)
                .with_children(|parent| {
                    parent.spawn_bundle(health_bar).insert(ValueBarComponent);
                })
                .insert(MaxBarComponent);
        })
        .insert(bar);
}

fn create_stat_text(asset_server: &Res<AssetServer>, value: &str) -> TextBundle {
    TextBundle {
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
            value,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Left,
                ..default()
            },
        ),
        ..default()
    }
}
