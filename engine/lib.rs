mod events;
mod pixel_type_matches;
mod physics;
mod grid;

use array2d::Array2D;
use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;
pub use self::events::*;
pub use self::physics::*;
pub use self::grid::*;

pub struct SandboxPlugin;
#[derive(Component)]
pub struct Pixel;

#[derive(Debug, Component)]
pub enum PixelType {
    Sand, Stone,
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


pub fn start(mut pb: QueryPixelBuffer) {
    // Set each pixel to a random color
    pb.frame().per_pixel(|_, _| bevy_pixel_buffer::pixel::Pixel::from(Color::NONE));
}
