use bevy::prelude::*;

use crate::{
    assets_manager::AssetsManager,
    day_cycle::DayCycleResource,
    player::Player,
    spells::SpellKind,
    states::GameStates,
    stats_window::ui_bars::{BarStatType, MaxBarComponent, UIBar, ValueBarComponent},
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStates::Game).with_system(setup_ui));
        app.add_system_set(SystemSet::on_update(GameStates::Game).with_system(update_ui));
    }
}

#[derive(Component)]
pub struct ActiveSpellHud;

#[derive(Component)]
pub struct DaysElapsedHud;

fn setup_ui(mut commands: Commands, assets: Res<AssetsManager>) {
    let root = NodeBundle {
        // transform: Transform::from_xyz(0., 0., 101.),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.),
                left: Val::Px(0.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            size: Size::new(Val::Px(400.), Val::Px(150.)),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::ColumnReverse,
            padding: Rect::all(Val::Px(10.)),
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    };

    let active_spell = TextBundle {
        text: Text::with_section(
            "Active spell: None",
            TextStyle {
                color: Color::WHITE.into(),
                font: assets.font_regular.clone(),
                font_size: 24.,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Left,
            },
        ),
        ..default()
    };

    let days_elapsed = TextBundle {
        style: Style { ..default() },
        text: Text::with_section(
            "Days survived: 0",
            TextStyle {
                color: Color::WHITE.into(),
                font: assets.font_regular.clone(),
                font_size: 24.,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Left,
            },
        ),
        ..default()
    };

    let health = TextBundle {
        style: Style { ..default() },
        text: Text::with_section(
            "Health",
            TextStyle {
                color: Color::WHITE.into(),
                font: assets.font_regular.clone(),
                font_size: 24.,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Left,
            },
        ),
        ..default()
    };

    commands.spawn_bundle(root).with_children(|parent| {
        parent.spawn_bundle(days_elapsed).insert(DaysElapsedHud);
        parent.spawn_bundle(active_spell).insert(ActiveSpellHud);
        parent.spawn_bundle(health);
        create_player_ui_bar(parent, UIBar::from_type(BarStatType::PlayerHealth));
    });
}

fn update_ui(
    mut q_active_spell: Query<&mut Text, (With<ActiveSpellHud>, Without<DaysElapsedHud>)>,
    mut q_days_elapsed: Query<&mut Text, (With<DaysElapsedHud>, Without<ActiveSpellHud>)>,
    day_cycle: Res<DayCycleResource>,
    q_player: Query<&Player>,
) {
    for player in q_player.iter() {
        for mut text in q_active_spell.iter_mut() {
            let active_spell = match player.active_spell {
                SpellKind::FireProjectile => "Fire projectile",
                SpellKind::SpawnChimera => "Spawn chimera",
            };

            text.sections[0].value = format!("Active spell: {}", active_spell);
        }
    }

    for mut text in q_days_elapsed.iter_mut() {
        text.sections[0].value = format!("Days survived: {}", day_cycle.days_passed);
    }
}

pub fn create_player_ui_bar(parent: &mut ChildBuilder, bar: UIBar) {
    let full_bar = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Px(15.)),
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
