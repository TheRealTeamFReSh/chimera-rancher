use bevy::{prelude::*, ui::widget::ImageMode};

use crate::{chimeras::ChimeraPartKind, player::Player};

pub struct InventoryUIPlugin;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui).add_system(update_ui);
    }
}

#[derive(Component)]
pub struct PartInventoryContainer;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            padding: Rect::all(Val::Px(5.)),
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    };

    let hotbar = ImageBundle {
        image: asset_server.load("hotbar.png").into(),
        image_mode: ImageMode::KeepAspect,
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(5.0),
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Px(5.0),
            },
            size: Size::new(Val::Auto, Val::Px(64.)),
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn_bundle(hotbar);
    commands
        .spawn_bundle(root)
        .with_children(|parent| {
            create_item_icon(&asset_server, parent, "chickenhead.png", true);
            create_item_icon(&asset_server, parent, "chickentail.png", false);
            create_item_icon(&asset_server, parent, "cowtail.png", false);
            create_item_icon(&asset_server, parent, "pighead.png", true);
        })
        .insert(PartInventoryContainer);
}

// TODO: if the inventory size is fixed, spawn N cells then change the texture
// visibility instead of despawning
fn update_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_container: Query<Entity, With<PartInventoryContainer>>,
    q_player: Query<&Player>,
) {
    for cont_ent in q_container.iter() {
        // start by cleaning
        let mut container = commands.entity(cont_ent);
        container.despawn_descendants();

        // add the parts
        container.with_children(|parent| {
            for player in q_player.iter() {
                for part in player.inventory.chimera_parts.iter() {
                    let is_head = matches!(part.kind, ChimeraPartKind::Head(_));
                    create_item_icon(&asset_server, parent, &part.texture, is_head)
                }
            }
        });
    }
}

fn create_item_icon(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    path: &str,
    is_head: bool,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(48.), Val::Px(48.)),
                overflow: Overflow::Hidden,
                margin: Rect {
                    left: Val::Px(6.),
                    right: Val::Px(10.),
                    top: Val::Px(5.),
                    bottom: Val::Px(8.5),
                },
                ..default()
            },
            // color: Color::RED.into(),
            color: Color::rgba_u8(10, 10, 10, 50).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                image: asset_server.load(path).into(),
                image_mode: ImageMode::KeepAspect,
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(if is_head { -48. } else { 0. }),
                        right: Val::Auto,
                        top: Val::Px(0.),
                        bottom: Val::Auto,
                    },
                    size: Size::new(Val::Px(96.), Val::Px(48.)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
