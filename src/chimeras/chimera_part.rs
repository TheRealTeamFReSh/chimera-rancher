use crate::animals::AnimalKind;
use bevy::prelude::*;

#[derive(Hash, PartialEq, Eq)]
pub enum ChimeraPartKind {
    Head(AnimalKind),
    Tail(AnimalKind),
}

#[derive(Component)]
pub struct ChimeraPartComponent;
