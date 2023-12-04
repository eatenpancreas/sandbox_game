use std::ops::Sub;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use rand::Rng;
use crate::{Grid, GridPos, PixelEventType, PixelType, SetPixelEvent};

pub struct SandboxPhysicsPlugin;
#[derive(Component)]
pub struct SandyPhysics;

impl SandyPhysics {
    pub fn physics(
        q_phys: Query<(&GridPos), With<SandyPhysics>>,
        grid: ResMut<Grid>,
        mut set_pixel: EventWriter<SetPixelEvent>,
    ) {
        for pos in q_phys.iter() {
            let new_y = (pos.0.y + 1).min(grid.0.num_rows() as u32 - 1);
            if let Some(&None) = grid.0.get(pos.0.x as usize, new_y as usize) {
                let mut rng = rand::thread_rng();
                match rng.gen_range(0.0..1.0) {
                    0.0..=0.33 => {
                        let new_x = (pos.0.x + 1).min(grid.0.num_columns() as u32 - 1);
                        if let Some(&None) = grid.0.get(new_x as usize, new_y as usize) {
                            set_pixel.send(SetPixelEvent(PixelEventType::Swap(pos.0, UVec2::new(new_x, new_y))));
                            return;
                        }
                    } 0.66..=1.0 => {
                        let new_x = (pos.0.x as i32 - 1).max(0) as u32;
                        if let Some(&None) = grid.0.get(new_x as usize, new_y as usize) {
                            set_pixel.send(SetPixelEvent(PixelEventType::Swap(pos.0, UVec2::new(new_x, new_y))));
                            return;
                        }
                    }
                    _ => {}
                }
                set_pixel.send(SetPixelEvent(PixelEventType::Swap(pos.0, UVec2::new(pos.0.x, new_y))))
            }
        }
    }
}

impl Plugin for SandboxPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                SandyPhysics::physics
            ));
    }
}