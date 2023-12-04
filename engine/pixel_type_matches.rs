use bevy::ecs::system::EntityCommands;
use bevy::math::UVec2;
use bevy::prelude::{Color, Commands};
use crate::{GridPos, Pixel, PixelType};

impl PixelType {
    pub(crate) fn to_col(&self) -> Color {
        match self {
            PixelType::Sand => Color::YELLOW,
            PixelType::Stone => Color::GRAY,
            _ => Color::NONE
        }
    }
    
    pub(crate) fn spawn(&self, cmd: &mut Commands, pos: UVec2) -> Option<EntityCommands> {
        match self {
            PixelType::Sand => Some(cmd.spawn((
                Pixel, GridPos(pos)
            ))),
            PixelType::Stone => Some(cmd.spawn((
                Pixel, GridPos(pos)
            ))),
            _ => None
        }
    }
}