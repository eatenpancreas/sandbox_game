mod events;
mod pixel_type_matches;
mod physics;
mod grid;

use std::ops::Deref;
use array2d::Array2D;
use bevy::prelude::*;
use bevy_pixel_buffer::builder::CustomSpriteBundle;
use bevy_pixel_buffer::prelude::*;
pub use self::events::*;
pub use self::physics::*;
pub use self::grid::*;

pub struct SandboxPlugin {
    pub size: UVec2,
    pub pixel_size: UVec2
}
#[derive(Component)]
pub struct Pixel;

#[derive(Debug, Component, Copy, Clone)]
#[derive(PartialEq)]
pub enum PixelType {
    Sand, Stone, Water, Metal, Dirt,
    Lava,
}

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        let size = PixelBufferSize {
            size: self.size,       // amount of pixels
            pixel_size: self.pixel_size, // size of each pixel in the screen
        };

        let grid = Array2D::filled_with(None, self.size.x as usize, self.size.y as usize);

        let buff = PixelBufferBuilder::new()
            .with_size(size)
            .with_render(RenderConfig::Sprite {
                spawn_camera: true,
                sprite_bundle: CustomSpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 10.0),
                    ..Default::default()
                },
            })
            .setup();

        app
            .insert_resource(Events::<GridClickEvent>::default())
            .insert_resource(Events::<SetPixelEvent>::default())
            .insert_resource(Grid(grid))
            .add_systems(PreStartup, buff)
            .add_systems(Startup, start)
            .add_systems(Update, (
                click_on_grid, set_pixel
            ));
    }
}


pub fn start(
    mut pb: QueryPixelBuffer,
    mut cmd: Commands
) {
    // Set each pixel to a random color
    pb.frame().per_pixel(|_, _| bevy_pixel_buffer::pixel::Pixel::from(Color::NONE));
    
    for buf in pb.deref().iter() {
        
        let mult = buf.pixel_buffer.size.size * buf.pixel_buffer.size.pixel_size;

        cmd.spawn(SpriteBundle {
            sprite: Sprite{ color: Color::DARK_GRAY, ..default() },
            transform: Transform::from_scale(mult.as_vec2().extend(0.)),
            ..default()
        });
    }
}


pub(crate) fn warn_on_err<T, E>(res: Result<T, E>) -> Option<T> where E: ToString {
    match res {
        Ok(i) => Some(i),
        Err(e) => { warn!("{}", e.to_string()); return None }
    }
}

