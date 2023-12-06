
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use rand::Rng;
use crate::{Grid, GridPos, PixelEventType, PixelType, SetPixelEvent};

pub struct SandboxPhysicsPlugin;
#[derive(Component)]
pub struct SandyPhysics {
    pub(crate) disperse_chance: f32,
    pub(crate) do_move_on_ground: bool,
    pub(crate) weight: u16,
}

#[derive(Component)]
pub struct ReactivePhysics {
    pub(crate) reacts_to: Vec<(Option<PixelType>, Reaction)>
}

pub enum Reaction {
    ChangeInto(Option<PixelType>)
}

#[derive(Component)]
pub struct PassablePhysics {
    pub(crate) weight: u16
}

impl SandyPhysics {
    pub fn physics(
        q_phys: Query<(&GridPos, &SandyPhysics)>,
        q_pass: Query<&PassablePhysics>,
        grid: Res<Grid>,
        mut set_pixel: EventWriter<SetPixelEvent>,
    ) {
        let mut rng = rand::thread_rng();
        for (pos, phys) in q_phys.iter() {
            if !phys.do_move_on_ground && pos.0.y == grid.0.num_rows() as u32 - 1 {
                continue;
            }
            
            let new_y = (pos.0.y + 1).min(grid.0.num_rows() as u32 - 1);

            let rn = rng.gen_range(0.0..1.0);

            if rn % 0.01 > 0.001 && new_y > pos.0.y && sand_drop(&mut set_pixel, &q_pass, &grid, pos.0, phys.weight, rn, pos.0.x, new_y) {
                continue;
            }
            
            if phys.do_move_on_ground && rng.gen_range(0..4) > 1 {
                do_sand_drop_to_sides(&mut set_pixel, &q_pass, &grid, phys, pos, rn, pos.0.y);
            } else if new_y > pos.0.y {
                do_sand_drop_to_sides(&mut set_pixel, &q_pass, &grid, phys, pos, rn, new_y);
            }
        }
    }
}

fn do_sand_drop_to_sides(
    set_pixel: &mut EventWriter<SetPixelEvent>,
    q_pass: &Query<&PassablePhysics>,
    grid: &Res<Grid>,
    phys: &SandyPhysics,
    pos: &GridPos,
    rn: f32,
    y: u32,
) {
    if let Some(x) = if (0.0..=phys.disperse_chance).contains(&rn) {
        Some((pos.0.x + 1).min(grid.0.num_columns() as u32 - 1))
    } else if ((1.0 - phys.disperse_chance)..=1.0).contains(&rn) {
        Some((pos.0.x as i32 - 1).max(0) as u32)
    } else { None } {
        sand_drop(set_pixel, &q_pass, &grid, pos.0, phys.weight, rn, x, y);
    }
}

fn sand_drop(
    set_pixel: &mut EventWriter<SetPixelEvent>,
    q_pass: &Query<&PassablePhysics>,
    grid: &Res<Grid>,
    from: UVec2,
    weight: u16,
    rn: f32,
    x: u32,
    y: u32,
) -> bool {
    if let Some(ent) = grid.0.get(x as usize, y as usize) {
        match ent {
            None => {
                set_pixel.send(SetPixelEvent(PixelEventType::Swap(from, UVec2::new(x, y)))); 
                return true 
            },
            Some(ent) => if let Ok(phys) = q_pass.get(*ent) {
                if (weight > phys.weight && rn % 1.0 > phys.weight as f32 / 100.0) {
                    set_pixel.send(SetPixelEvent(PixelEventType::Swap(from, UVec2::new(x, y))));
                    return true
                }
            }
        }
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