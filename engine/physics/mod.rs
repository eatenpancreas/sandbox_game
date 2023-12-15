mod sandy;
mod reactive;

pub use self::sandy::*;
pub use self::reactive::*;

use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use rand::Rng;
use crate::{Grid, GridPos, PixelEventType, PixelType, SetPixelEvent};

pub struct SandboxPhysicsPlugin;

impl Plugin for SandboxPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            SandyPhysics::physics
        );
    }
}