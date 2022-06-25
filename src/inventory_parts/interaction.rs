use bevy::prelude::*;

use crate::{
    assets_manager::AssetsManager,
    chimeras::{ChimeraPartAttributes, ChimeraPartKind},
};

use super::InventoryItem;

#[derive(Debug)]
pub struct InventoryManagement {
    pub last_inv_size: usize,
    pub is_selecting_first: bool,
    pub target_1: ItemTarget,
    pub target_2: ItemTarget,
}

#[derive(Debug)]
pub struct ItemTarget {
    pub selection: Option<(Entity, ChimeraPartAttributes)>,
    pub is_setup: bool,
    pub entity: Option<Entity>,
}

impl ItemTarget {
    pub fn reset(&mut self) {
        self.selection = None;
        self.is_setup = false;
        self.entity = None;
    }
}

impl Default for ItemTarget {
    fn default() -> Self {
        ItemTarget {
            selection: None,
            is_setup: false,
            entity: None,
        }
    }
}

impl InventoryManagement {
    pub fn reset(&mut self) {
        self.is_selecting_first = true;
        self.target_1.reset();
        self.target_2.reset();
    }
}

impl Default for InventoryManagement {
    fn default() -> Self {
        InventoryManagement {
            last_inv_size: 0,
            is_selecting_first: true,
            target_1: ItemTarget::default(),
            target_2: ItemTarget::default(),
        }
    }
}

pub fn handle_click(
    mut commands: Commands,
    q_inter: Query<(&Interaction, Entity, &InventoryItem), Changed<Interaction>>,
    mut inv_man: ResMut<InventoryManagement>,
) {
    for (interaction, entity, item) in q_inter.iter() {
        match *interaction {
            Interaction::Clicked => {
                // set selection
                if inv_man.is_selecting_first {
                    if let Some(entity) = inv_man.target_1.entity {
                        commands.entity(entity).despawn_recursive();
                        inv_man.target_1.reset();
                    }

                    inv_man.target_1.selection = Some((entity, item.part.clone()));
                } else {
                    if let Some(entity) = inv_man.target_2.entity {
                        commands.entity(entity).despawn_recursive();
                        inv_man.target_2.reset();
                    }

                    inv_man.target_2.selection = Some((entity, item.part.clone()));
                }
                inv_man.is_selecting_first = !inv_man.is_selecting_first;

                info!("Inv_man: {:?}", inv_man);
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct InventoryItemTarget;

pub fn set_selected_items(
    mut commands: Commands,
    mut inv_man: ResMut<InventoryManagement>,
    assets: Res<AssetsManager>,
) {
    let mut setup = |target: &mut ItemTarget| {
        if target.is_setup {
            return;
        }

        if let Some((target_entity, part)) = target.selection.clone() {
            let is_head = matches!(part.kind, ChimeraPartKind::Head(_));

            commands.entity(target_entity).with_children(|parent| {
                let child_id = parent
                    .spawn_bundle(ImageBundle {
                        image: assets.texture_target.clone().into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Px(0.),
                                bottom: Val::Auto,
                                left: {
                                    if is_head {
                                        Val::Auto
                                    } else {
                                        Val::Px(0.)
                                    }
                                },
                                right: {
                                    if !is_head {
                                        Val::Auto
                                    } else {
                                        Val::Px(0.)
                                    }
                                },
                            },
                            size: Size::new(Val::Px(48.), Val::Px(48.)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(InventoryItemTarget)
                    .id();

                target.entity = Some(child_id);
            });

            target.is_setup = true;
        }
    };

    setup(&mut inv_man.target_1);
    setup(&mut inv_man.target_2);
}
