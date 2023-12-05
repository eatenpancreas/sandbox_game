
use bevy::math::UVec2;
use bevy::prelude::{Color, Commands, Entity};
use crate::{GridPos, Pixel, PixelType};
use crate::physics::SandyPhysics;

impl PixelType {
    pub(crate) fn to_col(&self) -> Color {
        match self {
            PixelType::Sand => Color::YELLOW,
            PixelType::Stone => Color::GRAY,
            PixelType::Water => Color::BLUE,
            PixelType::Metal => Color::GRAY,
            PixelType::Dirt => Color::SALMON,
            PixelType::Lava => Color::RED,
            _ => Color::NONE
        }
    }
    
    pub(crate) fn spawn(&self, cmd: &mut Commands, pos: &UVec2) -> Option<Entity> {
        match self {
            PixelType::Sand => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.23,
                    do_move_on_ground: false,
                },
                PixelType::Sand
            )).id()),
            PixelType::Stone => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.002,
                    do_move_on_ground: false,
                }, 
                PixelType::Stone
            )).id()),
            PixelType::Water => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.23,
                    do_move_on_ground: true,
                },
                PixelType::Water
            )).id()),
            PixelType::Metal => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                PixelType::Metal
            )).id()),
            PixelType::Dirt => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.02,
                    do_move_on_ground: false,
                },
                PixelType::Dirt
            )).id()),
            PixelType::Lava => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.02,
                    do_move_on_ground: true,
                },
                PixelType::Lava
            )).id()),
            _ => None
        }
    }
}