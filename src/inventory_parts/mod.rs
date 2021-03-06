use bevy::{prelude::*, ui::widget::ImageMode};

use crate::{
    assets_manager::AssetsManager,
    chimeras::{ChimeraPartAttributes, ChimeraPartKind},
    constants,
    player::Player,
    states::GameStates,
};

use self::interaction::InventoryManagement;

pub mod interaction;

pub struct InventoryUIPlugin;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InventoryManagement::default());

        // on enter
        app.add_system_set(SystemSet::on_enter(GameStates::Game).with_system(setup_ui));

        // on update
        app.add_system_set(
            SystemSet::on_update(GameStates::Game)
                .with_system(update_ui)
                .with_system(interaction::handle_click)
                .with_system(interaction::set_selected_items),
        );
    }
}

#[derive(Component)]
pub struct PartInventoryContainer;

fn setup_ui(mut commands: Commands, assets: Res<AssetsManager>) {
    let root = NodeBundle {
        transform: Transform::from_xyz(0., 0., constants::Z_UI),
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            padding: Rect::all(Val::Px(5.)),
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    };

    let hotbar = ImageBundle {
        image: assets.texture_hotbar.clone().into(),
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
        // .with_children(|parent| {
        //     create_item_icon(&asset_server, parent, "chickenhead.png", true);
        //     create_item_icon(&asset_server, parent, "chickentail.png", false);
        //     create_item_icon(&asset_server, parent, "cowtail.png", false);
        //     create_item_icon(&asset_server, parent, "pighead.png", true);
        // })
        .insert(PartInventoryContainer);
}

// TODO: if the inventory size is fixed, spawn N cells then change the texture
// visibility instead of despawning
fn update_ui(
    mut commands: Commands,
    q_container: Query<Entity, With<PartInventoryContainer>>,
    q_player: Query<&Player>,
    mut inv_man: ResMut<InventoryManagement>,
) {
    for player in q_player.iter() {
        // if inventory changed
        if inv_man.last_inv_size != player.inventory.chimera_parts.len() {
            // set new inventory length
            inv_man.last_inv_size = player.inventory.chimera_parts.len();

            // reset the references
            inv_man.reset();

            for cont_ent in q_container.iter() {
                // start by cleaning
                let mut container = commands.entity(cont_ent);
                container.despawn_descendants();

                // add the parts
                container.with_children(|parent| {
                    for part in player.inventory.chimera_parts.iter() {
                        let is_head = matches!(part.kind, ChimeraPartKind::Head(_));
                        create_item_icon(parent, &part.texture, is_head, part.clone())
                    }
                });
            }
        }
    }
}

#[derive(Component)]
pub struct InventoryItem {
    pub part: ChimeraPartAttributes,
}

fn create_item_icon(
    parent: &mut ChildBuilder,
    handle: &Handle<Image>,
    is_head: bool,
    part: ChimeraPartAttributes,
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
            parent
                .spawn_bundle(ButtonBundle {
                    image: handle.clone().into(),
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
                })
                .insert(InventoryItem { part });
        });
}
