
use bevy::math::UVec2;
use bevy::prelude::{Color, Commands, Entity};
use crate::{GridPos, PassablePhysics, Pixel, PixelType};
use crate::physics::SandyPhysics;

impl PixelType {
    pub(crate) fn to_col(&self) -> Color {  
        match self {
            PixelType::Sand => Color::hex("#ECC713").unwrap(),
            PixelType::Stone => Color::hex("#B8C7CF").unwrap(),
            PixelType::Water => Color::hex("#1338EC").unwrap(),
            PixelType::Metal => Color::hex("#8B97B0").unwrap(),
            PixelType::Dirt => Color::hex("#D4926A").unwrap(),
            PixelType::Lava => Color::hex("#FF4355").unwrap(),
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
                    weight: 30,
                },
                PixelType::Sand
            )).id()),
            
            PixelType::Stone => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.002,
                    do_move_on_ground: false,
                    weight: 70,
                },
                PixelType::Stone
            )).id()),
            
            PixelType::Water => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.23,
                    do_move_on_ground: true,
                    weight: 10,
                },
                PassablePhysics {
                    weight: 10,
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
                    weight: 30,
                },
                PixelType::Dirt
            )).id()),
            
            PixelType::Lava => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.02,
                    do_move_on_ground: true,
                    weight: 50,
                },
                PassablePhysics {
                    weight: 50,
                },
                PixelType::Lava
            )).id()),
        }
    }
}