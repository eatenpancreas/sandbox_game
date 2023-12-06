
use bevy::math::UVec2;
use bevy::prelude::{Color, Commands, Entity};
use crate::{GridPos, PassablePhysics, Pixel, PixelType, Reaction, ReactivePhysics};
use crate::physics::SandyPhysics;
use crate::PixelType::*;

impl PixelType {
    pub(crate) fn to_col(&self) -> Color {  
        match self {
            Sand => Color::hex("#ECC713").unwrap(),
            Stone => Color::hex("#B8C7CF").unwrap(),
            Water => Color::hex("#1338EC").unwrap(),
            Metal => Color::hex("#8B97B0").unwrap(),
            Dirt => Color::hex("#D4926A").unwrap(),
            Lava => Color::hex("#FF4355").unwrap(),
        }
    }
    
    pub(crate) fn spawn(&self, cmd: &mut Commands, pos: &UVec2) -> Option<Entity> {
        match self {
            Sand => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.23,
                    do_move_on_ground: false,
                    weight: 30,
                },
                Sand
            )).id()),
            
            Stone => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.002,
                    do_move_on_ground: false,
                    weight: 70,
                },
                Stone,
                ReactivePhysics { reacts_to: vec![
                    (Some(Water), Reaction::ChangeInto(Some(Sand))),
                    (Some(Lava), Reaction::ChangeInto(Some(Lava)))
                ],}
            )).id()),
            
            Water => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.49,
                    do_move_on_ground: true,
                    weight: 10,
                },
                PassablePhysics {
                    weight: 10,
                },
                Water
            )).id()),
            
            Metal => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                Metal
            )).id()),
            
            Dirt => Some(cmd.spawn((
                Pixel,
                GridPos(*pos),
                SandyPhysics {
                    disperse_chance: 0.02,
                    do_move_on_ground: false,
                    weight: 30,
                },
                Dirt
            )).id()),
            
            Lava => Some(cmd.spawn((
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
                Lava
            )).id()),
        }
    }
}