use bevy::prelude::*;

#[derive(Debug)]
pub enum BarStatType {
    Acceleration,
    Deceleration,
    Speed,
    Attack,
    Health,
    Regen,
    Range,
    PlayerHealth,
}

#[derive(Component)]
pub struct UIBar {
    pub max_possible_value: f32,
    pub max_value: f32,
    pub value: f32,
    pub bartype: BarStatType,
}

impl UIBar {
    pub fn from_type(bartype: BarStatType) -> Self {
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
