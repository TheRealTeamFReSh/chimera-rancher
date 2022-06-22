use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use self::chimera_part::{ChimeraPartComponent, ChimeraPartKind};
use crate::animals::AnimalKind;

mod chimera_part;

pub struct ChimerasPlugin;

impl Plugin for ChimerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(chimera_test_system);
    }
}

pub fn chimera_test_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_chimera(
        (
            ChimeraPartKind::Head(AnimalKind::Pig),
            ChimeraPartKind::Head(AnimalKind::Pig),
        ),
        Vec2::new(0.0, 0.0),
        &mut commands,
        &asset_server,
    )
}

#[derive(Default)]
pub struct ChimeraStats {
    speed: f32,
}

pub fn spawn_chimera(
    chimera_parts: (ChimeraPartKind, ChimeraPartKind),
    position: Vec2,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    let chimera_stats = ChimeraStats::default();

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
}
