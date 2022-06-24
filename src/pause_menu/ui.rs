use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle, NinePatchData};

use crate::pause_menu::button::UIButton;

// building the UI of the console
pub fn build_ui(
    mut commands: Commands,
    mut nine_patches: ResMut<Assets<NinePatchBuilder>>,
    asset_server: Res<AssetServer>,
) {
    info!("[PauseMenuPlugin] Building console UI");

    let background_texture_handle: Handle<Image> = asset_server.load("ui_background.png");
    let background_nine_patch_handle =
        nine_patches.add(NinePatchBuilder::by_margins(20, 20, 20, 20));

    let font_handle: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    // ---------- UI COMPONENTS ----------//

    // root component
    let parent_component = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect::all(Val::Px(0.)),
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 150).into(),
        ..Default::default()
    };

    // container
    let container = NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            size: Size::new(Val::Px(500.), Val::Px(400.)),
            ..Default::default()
        },
        color: Color::rgba(0., 0., 0., 0.).into(),
        ..Default::default()
    };

    // background
    let background = NinePatchBundle {
        style: Style {
            position_type: PositionType::Absolute,
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(500.), Val::Px(400.)),
            ..Default::default()
        },
        nine_patch_data: NinePatchData {
            nine_patch: background_nine_patch_handle,
            texture: background_texture_handle,
            ..Default::default()
        },
        ..Default::default()
    };

    // pause title
    let pause_title = TextBundle {
        text: Text {
            sections: vec![TextSection {
                style: TextStyle {
                    font: font_handle.clone(),
                    font_size: 64.,
                    color: Color::rgb_u8(205, 205, 205).into(),
                    ..Default::default()
                },
                value: "Pause".to_string(),
                ..Default::default()
            }],
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        },
        ..Default::default()
    };

    let resume_btn = UIButton::new(
        "Resume".to_string(),
        font_handle.clone(),
        "resume".to_string(),
    );
    let quit_btn = UIButton::new("Quit".to_string(), font_handle.clone(), "quit".to_string());

    // ---------- UI TREE CONSTRUCTION ----------//

    commands
        .spawn_bundle(parent_component)
        .with_children(|parent| {
            parent.spawn_bundle(background);
            parent.spawn_bundle(container).with_children(|parent| {
                parent.spawn_bundle(pause_title);
                resume_btn.spawn(parent);
                quit_btn.spawn(parent);
            });
        })
        .insert(super::PauseMenuEntity);

    info!("[PauseMenuPlugin] UI constructed");
}
