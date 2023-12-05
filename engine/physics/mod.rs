
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use rand::Rng;
use crate::{Grid, GridPos, PixelEventType, SetPixelEvent};

pub struct SandboxPhysicsPlugin;
#[derive(Component)]
pub struct SandyPhysics {
    pub(crate) disperse_chance: f32,
    pub(crate) do_move_on_ground: bool,
}

impl SandyPhysics {
    pub fn physics(
        q_phys: Query<(&GridPos, &SandyPhysics)>,
        grid: Res<Grid>,
        mut set_pixel: EventWriter<SetPixelEvent>,
    ) {
        let mut rng = rand::thread_rng();
        for (pos, phys) in q_phys.iter() {
            if !phys.do_move_on_ground && pos.0.y == grid.0.num_rows() as u32 - 1 {
                continue;
            }
            
            let new_y = (pos.0.y + 1).min(grid.0.num_rows() as u32 - 1);
            
            if sand_drop(&mut set_pixel, &grid, pos.0, pos.0.x, new_y) {
                continue;
            }
            
            let rn = rng.gen_range(0.0..1.0);
            if phys.do_move_on_ground && rng.gen_range(0..2) == 1 {
                do_sand_drop_to_sides(&mut set_pixel, &grid, phys, pos, rn, pos.0.y);
            } else {
                do_sand_drop_to_sides(&mut set_pixel, &grid, phys, pos, rn, new_y);
            }
        }
    }
}

fn do_sand_drop_to_sides(
    set_pixel: &mut EventWriter<SetPixelEvent>,
    grid: &Res<Grid>,
    phys: &SandyPhysics,
    pos: &GridPos,
    rn: f32,
    y: u32,
) {
    if (0.0..=phys.disperse_chance).contains(&rn) {
        sand_drop(set_pixel, &grid, pos.0, (pos.0.x + 1).min(grid.0.num_columns() as u32 - 1), y);
    } else if ((1.0 - phys.disperse_chance)..=1.0).contains(&rn) {
        sand_drop(set_pixel, &grid, pos.0, (pos.0.x as i32 - 1).max(0) as u32, y);
    }
}

fn sand_drop(
    set_pixel: &mut EventWriter<SetPixelEvent>,
    grid: &Res<Grid>,
    from: UVec2,
    x: u32,
    y: u32,
) -> bool {
    if let Some(&None) = grid.0.get(x as usize, y as usize) {
        set_pixel.send(SetPixelEvent(PixelEventType::Swap(from, UVec2::new(x, y))));
        return true;
    }
    false
}

impl Plugin for SandboxPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            SandyPhysics::physics
        );
    }
}