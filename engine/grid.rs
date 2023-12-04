use array2d::Array2D;
use bevy::math::UVec2;
use bevy::prelude::{Component, Entity, Resource};

#[derive(Resource)]
pub struct Grid(pub Array2D<Option<Entity>>);


#[derive(Component)]
pub struct GridPos(pub UVec2);

