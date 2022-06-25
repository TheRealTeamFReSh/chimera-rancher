use bevy::{app::AppExit, prelude::*};

use crate::{
    assets_manager::AssetsManager,
    day_cycle::DayCycleResource,
    pause_menu::button::{UIButton, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    states::GameStates,
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStates::GameOver).with_system(setup_ui));

        app.add_system_set(SystemSet::on_update(GameStates::GameOver).with_system(button_handler));
    }
}

#[derive(Component)]
pub struct GameOverScreenUI;

fn setup_ui(mut commands: Commands, assets: Res<AssetsManager>, day_cycle: Res<DayCycleResource>) {
    let root = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: Color::BLACK.into(),
        ..default()
    };

    let game_over_text = TextBundle {
        text: Text::with_section(
            "Game Over",
            TextStyle {
                font_size: 96.,
                font: assets.font_bold.clone(),
                color: Color::rgb_u8(220, 220, 220).into(),
            },
            TextAlignment {
                ..Default::default()
            },
        ),
        ..Default::default()
    };

    let score_text = TextBundle {
        text: Text::with_section(
            format!("You survived : {} days", day_cycle.days_passed),
            TextStyle {
                font_size: 48.,
                font: assets.font_regular.clone(),
                color: Color::rgb_u8(220, 220, 220).into(),
            },
            TextAlignment {
                ..Default::default()
            },
        ),
        ..Default::default()
    };

    let btn_group = NodeBundle {
        style: Style {
            margin: Rect {
                top: Val::Px(100.),
                ..Default::default()
            },
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    let quit_btn = UIButton::new(
        "Quit game".to_string(),
        assets.font_regular.clone(),
        "quit".to_string(),
    );

    commands
        .spawn_bundle(root)
        .with_children(|parent| {
            parent.spawn_bundle(game_over_text);
            parent.spawn_bundle(score_text);
            parent.spawn_bundle(btn_group).with_children(|parent| {
                //restart_btn.spawn(parent);
                quit_btn.spawn(parent);
            });
        })
        .insert(GameOverScreenUI);
}

pub fn button_handler(
    mut interaction_query: Query<(&Interaction, &mut UiColor, &UIButton), Changed<Interaction>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match button.name.as_str() {
                    "quit" => {
                        //game_state.set(GameStates::MainMenu).unwrap();
                        exit.send(AppExit);
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
