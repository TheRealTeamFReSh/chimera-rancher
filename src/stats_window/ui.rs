use bevy::{log, prelude::*};
use bevy_tweening::{lens::UiPositionLens, Animator, EaseFunction, Tween, TweeningType};

use crate::{
    animals::AnimalComponent,
    assets_manager::AssetsManager,
    chimeras::ChimeraComponent,
    constants::{self, MaxStats},
    health::Health,
};

use super::{ui_bars::*, EntityType, StatsWindow};

#[derive(Component)]
pub struct StatsWindowUI;

#[derive(Component)]
pub struct StatWindowTitle;

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
    mut stats_window: ResMut<StatsWindow>,
    maxi_stats: Res<MaxStats>,
    q_chimera: Query<(&Health, &ChimeraComponent)>,
    q_animal: Query<(&Health, &AnimalComponent)>,
    q_ui_bar: Query<(&Children, &UIBar), (Without<MaxBarComponent>, Without<ValueBarComponent>)>,
    mut q_ui_bar_max: Query<
        (&Children, &mut Style),
        (With<MaxBarComponent>, Without<ValueBarComponent>),
    >,
    mut q_ui_bar_value: Query<&mut Style, With<ValueBarComponent>>,
) {
    if let Some(target_entity) = stats_window.target {
        // get the player
        let zero_health = Health {
            health: 0.,
            max_health: 0.,
            regen: 0.,
            regen_timer: Timer::from_seconds(0.0, false),
        };
        // get the stats
        let (health, accel, decel, attack, speed, regen, range) =
            if stats_window.target_type == EntityType::Chimera {
                if let Ok((health, chimera)) = q_chimera.get(target_entity) {
                    let stats = chimera.stats;
                    (
                        health,
                        stats.accel,
                        stats.decel,
                        stats.attack,
                        stats.speed,
                        stats.regen,
                        stats.range,
                    )
                } else {
                    (&zero_health, 0., 0., 0., 0., 0., 0.)
                }
            } else if let Ok((health, animal)) = q_animal.get(target_entity) {
                let stats = animal.stats;
                (
                    health,
                    stats.accel,
                    stats.decel,
                    stats.attack,
                    stats.speed,
                    stats.regen,
                    stats.range,
                )
            } else {
                (&zero_health, 0., 0., 0., 0., 0., 0.)
            };

        // if the values are null, the entity does not exist anymore
        if health.max_health == 0. && health.health == 0. {
            stats_window.target = None;
            stats_window.cursor = None;
            stats_window.target_type = EntityType::None;
            return;
        }

        // for each bar
        for (children, bar) in q_ui_bar.iter() {
            // set value according to bartype
            let (max_value_possible, max_value, value) = match bar.bartype {
                BarStatType::Acceleration => (maxi_stats.accel, accel, accel),
                BarStatType::Deceleration => (maxi_stats.decel, decel, decel),
                BarStatType::Speed => (maxi_stats.speed, speed, speed),
                BarStatType::Attack => (maxi_stats.attack, attack, attack),
                BarStatType::Health => (maxi_stats.health, health.max_health, health.health),
                BarStatType::Regen => (maxi_stats.regen, regen, regen),
                BarStatType::Range => (maxi_stats.range, range, range),
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

pub fn setup_ui(mut commands: Commands, assets: Res<AssetsManager>) {
    let container = NodeBundle {
        transform: Transform::from_xyz(0., 0., constants::Z_UI),
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
            size: Size::new(Val::Px(300.0), Val::Px(550.0)),
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
                font: assets.font_bold.clone(),
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
                        parent.spawn_bundle(content_text).insert(StatWindowTitle);
                        // health
                        parent.spawn_bundle(create_stat_text(&assets, "Health"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Health));
                        // speed
                        parent.spawn_bundle(create_stat_text(&assets, "Speed"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Speed));
                        // accel
                        parent.spawn_bundle(create_stat_text(&assets, "Acceleration"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Acceleration));
                        // decel
                        parent.spawn_bundle(create_stat_text(&assets, "Deceleration"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Deceleration));
                        // attack
                        parent.spawn_bundle(create_stat_text(&assets, "Attack"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Attack));
                        // regen
                        parent.spawn_bundle(create_stat_text(&assets, "Regeneration"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Regen));
                        // range
                        parent.spawn_bundle(create_stat_text(&assets, "Range"));
                        create_ui_bar(parent, UIBar::from_type(BarStatType::Range));
                    });
            })
            .insert(Animator::<Style>::default())
            .insert(StatsWindowUI);
    });
}

pub fn display_stats_window(
    mut stats_window: ResMut<StatsWindow>,
    mut q_title: Query<&mut Text, With<StatWindowTitle>>,
    mut q_anim: Query<&mut Animator<Style>, With<StatsWindowUI>>,
) {
    // get animator
    let mut animator = q_anim.get_single_mut().unwrap();

    // if finished
    if stats_window.target != None && !stats_window.opened {
        log::debug!("opening stat window");

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
        log::debug!("closing stat window");

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

    // change the title
    for mut title in q_title.iter_mut() {
        title.sections[0].value = (match stats_window.target_type {
            EntityType::Animal => "Animal stats",
            EntityType::Chimera => "Chimera stats",
            _ => &title.sections[0].value,
        })
        .to_string();
    }
}

fn create_stat_text(assets: &Res<AssetsManager>, value: &str) -> TextBundle {
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
                font: assets.font_regular.clone(),
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
