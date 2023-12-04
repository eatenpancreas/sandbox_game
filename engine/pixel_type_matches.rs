
use bevy::math::UVec2;
use bevy::prelude::{Color, Commands, Entity};
use crate::{GridPos, Pixel, PixelType};
use crate::physics::SandyPhysics;

impl PixelType {
    pub(crate) fn to_col(&self) -> Color {
        match self {
            PixelType::Sand => Color::YELLOW,
            PixelType::Stone => Color::GRAY,
            _ => Color::NONE
        }
    }
    
    pub(crate) fn spawn(&self, cmd: &mut Commands, pos: &UVec2) -> Option<Entity> {
        match self {
            PixelType::Sand => Some(cmd.spawn((
                Pixel, GridPos(*pos), SandyPhysics, PixelType::Sand
            )).id()),
            PixelType::Stone => Some(cmd.spawn((
                Pixel, GridPos(*pos), PixelType::Stone
            )).id()),
            _ => None
        }
    }
}