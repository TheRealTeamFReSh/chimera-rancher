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
                margin: Rect::all(Val::Px(5.)),
                ..default()
            },
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
