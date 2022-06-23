use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

use self::chimera_part::ChimeraPartKind;
use crate::animals::AnimalKind;

mod chimera_part;

#[allow(dead_code)]
#[derive(Component)]
pub struct ChimeraComponent {
    stats: ChimeraStats,
}

#[allow(dead_code)]
pub struct ChimeraStats {
    speed: f32,
    accel: f32,
    decel: f32,
}

pub struct ChimeraPartAttributes {
    speed: f32,
    accel: f32,
    decel: f32,
    collider_size: Vec2,
    texture: String,
}

type ChimeraPartAttributesResource = HashMap<ChimeraPartKind, ChimeraPartAttributes>;

pub struct ChimerasPlugin;

impl Plugin for ChimerasPlugin {
    fn build(&self, app: &mut App) {
        let mut chimera_part_attr_res = ChimeraPartAttributesResource::default();
        chimera_part_attr_res.insert(
            ChimeraPartKind::Head(AnimalKind::Pig),
            ChimeraPartAttributes {
                speed: 60.0,
                accel: 1.5,
                decel: 7.0,
                collider_size: Vec2::new(12.0, 10.0),
                texture: "pighead.png".to_string(),
            },
        );
        chimera_part_attr_res.insert(
            ChimeraPartKind::Tail(AnimalKind::Pig),
            ChimeraPartAttributes {
                speed: 60.0,
                accel: 1.5,
                decel: 7.0,
                collider_size: Vec2::new(12.0, 10.0),
                texture: "pigtail.png".to_string(),
            },
        );
        chimera_part_attr_res.insert(
            ChimeraPartKind::Head(AnimalKind::Cow),
            ChimeraPartAttributes {
                speed: 50.0,
                accel: 1.75,
                decel: 7.0,
                collider_size: Vec2::new(12.0, 10.0),
                texture: "cowhead.png".to_string(),
            },
        );
        chimera_part_attr_res.insert(
            ChimeraPartKind::Tail(AnimalKind::Cow),
            ChimeraPartAttributes {
                speed: 50.0,
                accel: 1.75,
                decel: 7.0,
                collider_size: Vec2::new(12.0, 10.0),
                texture: "cowtail.png".to_string(),
            },
        );
        chimera_part_attr_res.insert(
            ChimeraPartKind::Head(AnimalKind::Dog),
            ChimeraPartAttributes {
                speed: 80.0,
                accel: 2.2,
                decel: 7.0,
                collider_size: Vec2::new(12.0, 10.0),
                texture: "doghead.png".to_string(),
            },
        );
        chimera_part_attr_res.insert(
            ChimeraPartKind::Tail(AnimalKind::Dog),
            ChimeraPartAttributes {
                speed: 80.0,
                accel: 2.2,
                decel: 7.0,
                collider_size: Vec2::new(12.0, 10.0),
                texture: "dogtail.png".to_string(),
            },
        );
        chimera_part_attr_res.insert(
            ChimeraPartKind::Head(AnimalKind::Chicken),
            ChimeraPartAttributes {
                speed: 70.0,
                accel: 2.0,
                decel: 7.0,
                collider_size: Vec2::new(6.0, 10.0),
                texture: "chickenhead.png".to_string(),
            },
        );
        chimera_part_attr_res.insert(
            ChimeraPartKind::Tail(AnimalKind::Chicken),
            ChimeraPartAttributes {
                speed: 70.0,
                accel: 2.0,
                decel: 7.0,
                collider_size: Vec2::new(6.0, 10.0),
                texture: "chickentail.png".to_string(),
            },
        );
        chimera_part_attr_res.insert(
            ChimeraPartKind::Head(AnimalKind::Horse),
            ChimeraPartAttributes {
                speed: 100.0,
                accel: 3.0,
                decel: 7.0,
                collider_size: Vec2::new(12.0, 10.0),
                texture: "horsehead.png".to_string(),
            },
        );
        chimera_part_attr_res.insert(
            ChimeraPartKind::Tail(AnimalKind::Horse),
            ChimeraPartAttributes {
                speed: 100.0,
                accel: 3.0,
                decel: 7.0,
                collider_size: Vec2::new(12.0, 10.0),
                texture: "horsetail.png".to_string(),
            },
        );

        app.insert_resource(chimera_part_attr_res)
            .add_startup_system(chimera_test_system);
    }
}

pub fn chimera_test_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    chimera_part_attr_res: Res<ChimeraPartAttributesResource>,
) {
    spawn_chimera(
        (
            ChimeraPartKind::Head(AnimalKind::Pig),
            ChimeraPartKind::Head(AnimalKind::Pig),
        ),
        Vec2::new(0.0, 0.0),
        &chimera_part_attr_res,
        &mut commands,
        &asset_server,
    );

    spawn_chimera(
        (
            ChimeraPartKind::Tail(AnimalKind::Cow),
            ChimeraPartKind::Tail(AnimalKind::Cow),
        ),
        Vec2::new(-20.0, -60.0),
        &chimera_part_attr_res,
        &mut commands,
        &asset_server,
    );

    spawn_chimera(
        (
            ChimeraPartKind::Head(AnimalKind::Pig),
            ChimeraPartKind::Tail(AnimalKind::Chicken),
        ),
        Vec2::new(20.0, 60.0),
        &chimera_part_attr_res,
        &mut commands,
        &asset_server,
    );

    spawn_chimera(
        (
            ChimeraPartKind::Head(AnimalKind::Chicken),
            ChimeraPartKind::Head(AnimalKind::Dog),
        ),
        Vec2::new(-120.0, 50.0),
        &chimera_part_attr_res,
        &mut commands,
        &asset_server,
    );
}

pub fn spawn_chimera(
    chimera_parts: (ChimeraPartKind, ChimeraPartKind),
    position: Vec2,
    chimera_part_attr_res: &ChimeraPartAttributesResource,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    // TODO: prevent/correct player from creating normal animals and backwards animals

    let head_attributes = &chimera_part_attr_res[&chimera_parts.0];
    let tail_attributes = &chimera_part_attr_res[&chimera_parts.1];

    commands
        .spawn()
        .insert_bundle(TransformBundle::from(Transform::from_translation(
            position.extend(0.0),
        )))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(ChimeraComponent {
            stats: ChimeraStats {
                speed: head_attributes.speed + tail_attributes.speed,
                accel: head_attributes.accel + tail_attributes.accel,
                decel: head_attributes.decel + tail_attributes.decel,
            },
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            head_attributes.collider_size.x + tail_attributes.collider_size.x,
            head_attributes.collider_size.y,
        ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(&head_attributes.texture),
                    sprite: Sprite {
                        flip_x: matches!(chimera_parts.0, ChimeraPartKind::Tail(_)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Transform::from_xyz(
                    tail_attributes.collider_size.x - head_attributes.collider_size.x,
                    0.0,
                    0.0,
                ));

            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(&tail_attributes.texture),
                    sprite: Sprite {
                        flip_x: matches!(chimera_parts.1, ChimeraPartKind::Head(_)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Transform::from_xyz(
                    tail_attributes.collider_size.x - head_attributes.collider_size.x,
                    0.0,
                    0.0,
                ));
        });

    /*
    let mut chimera_entity = commands.spawn();
    match chimera_parts.0 {
        ChimeraPartKind::Head(animal_kind) => {
            // give this chimera part the collider
            // flip sprite if head part
            match animal_kind {
                crate::animals::AnimalKind::Pig => {
                    chimera_entity
                        .insert_bundle(SpriteBundle {
                            texture: asset_server.load("pighead.png"),
                            sprite: Sprite {
                                flip_x: true,
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Transform::from_translation(position.extend(0.0)))
                        .insert(Velocity::default())
                        .insert(ChimeraPartComponent)
                        .insert(RigidBody::Dynamic)
                        .insert(Collider::cuboid(25.0, 10.0))
                        .insert(LockedAxes::ROTATION_LOCKED);
                }
                crate::animals::AnimalKind::Cow => todo!(),
                crate::animals::AnimalKind::Dog => todo!(),
                crate::animals::AnimalKind::Horse => todo!(),
                crate::animals::AnimalKind::Chicken => todo!(),
            }
        }
        ChimeraPartKind::Tail(animal_kind) => match animal_kind {
            crate::animals::AnimalKind::Pig => todo!(),
            crate::animals::AnimalKind::Cow => todo!(),
            crate::animals::AnimalKind::Dog => todo!(),
            crate::animals::AnimalKind::Horse => todo!(),
            crate::animals::AnimalKind::Chicken => todo!(),
        },
    };

    chimera_entity.with_children(|parent| {
        match chimera_parts.1 {
            ChimeraPartKind::Head(animal_kind) => match animal_kind {
                AnimalKind::Pig => parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("pighead.png"),
                    ..default()
                }),
                AnimalKind::Cow => todo!(),
                AnimalKind::Dog => todo!(),
                AnimalKind::Horse => todo!(),
                AnimalKind::Chicken => todo!(),
            },
            ChimeraPartKind::Tail(animal_kind) => match animal_kind {
                AnimalKind::Pig => todo!(),
                AnimalKind::Cow => todo!(),
                AnimalKind::Dog => todo!(),
                AnimalKind::Horse => todo!(),
                AnimalKind::Chicken => todo!(),
            },
        };
    });
    */
}
