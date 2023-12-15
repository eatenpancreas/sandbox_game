use bevy::prelude::Component;
use crate::PixelType;

#[derive(Component)]
pub struct ReactivePhysics {
    pub(crate) reacts_to: Vec<(Option<PixelType>, Reaction)>
}

pub enum Reaction {
    ChangeInto(Option<PixelType>)
}

impl ReactivePhysics {
    pub fn physics(
        
    ) {
        
    }
}