mod events;
mod pixel_type_matches;

use array2d::Array2D;
use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;
pub use self::events::*;

pub struct SandboxPlugin;
#[derive(Resource)]
pub struct Grid(Array2D<Option<Entity>>);
#[derive(Component)]
pub struct GridPos(pub UVec2);
#[derive(Component)]
pub struct Pixel;

#[derive(Debug)]
pub enum PixelType {
    Sand, Stone, 
    None
}

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        let size = PixelBufferSize {
            size: UVec2::new(32, 32),       // amount of pixels
            pixel_size: UVec2::new(16, 16), // size of each pixel in the screen
        };

        let grid = Array2D::filled_with(None, 32, 32);

        app
            .insert_resource(Events::<GridClickEvent>::default())
            .insert_resource(Events::<SetPixelEvent>::default())
            .insert_resource(Grid(grid))
            .add_systems(PreStartup, pixel_buffer_setup(size))
            .add_systems(Startup, start)
            .add_systems(Update, (
                click_on_grid, set_pixel
            ));
    }
}
