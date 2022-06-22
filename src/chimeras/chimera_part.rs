use crate::animals::AnimalKind;
use bevy::prelude::*;

pub enum ChimeraPartKind {
    Head(AnimalKind),
    Tail(AnimalKind),
}

#[derive(Component)]
pub struct ChimeraPartComponent;
