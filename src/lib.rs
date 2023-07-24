use bevy::prelude::Component;

pub mod constants;
pub mod ui;

#[derive(Component)]
pub struct Karma(pub i32);

#[derive(Component)]
pub struct NPC;

#[derive(Component)]
pub struct Player;
