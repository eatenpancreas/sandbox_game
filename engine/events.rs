
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_pixel_buffer::prelude::*;
use crate::{Grid, PixelType};

#[derive(Event, Debug)]
pub struct GridClickEvent(pub UVec2);
#[derive(Event, Debug)]
pub struct SetPixelEvent(pub UVec2, pub PixelType);

pub(crate) fn click_on_grid(
    buttons: Res<Input<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_buffer: Query<&PixelBuffer>,
    mut ev_grid_click: EventWriter<GridClickEvent>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = q_window.single();
        let (camera, camera_transform) = q_camera.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            for buffer in q_buffer.iter() {
                let mut grid_position = (world_position /
                    buffer.size.pixel_size.as_vec2()
                    + (buffer.size.size.as_vec2() / 2.0)).floor().as_uvec2();

                grid_position.y = buffer.size.size.y - grid_position.y - 1;

                if grid_position.x >= 0 && grid_position.y >= 0
                    && grid_position.x < buffer.size.size.x
                    && grid_position.y < buffer.size.size.y {
                    ev_grid_click.send(GridClickEvent(grid_position));
                }
            }
        }
    }
}

pub(crate) fn set_pixel(
    mut r_pixel: EventReader<SetPixelEvent>,
    mut pb: QueryPixelBuffer,
    grid: Res<Grid>,
    mut cmd: Commands
) {
    let r = r_pixel.read();
    if r.len() <= 0 { return; }
    let mut frame = pb.frame();

    for SetPixelEvent(vec, pixel) in r {
        let entity = pixel.spawn(&mut cmd, *vec)
            .and_then(|ec| Some(ec.id()));
        let _ = grid.0.set(vec.x as usize, vec.y as usize, entity);
        let _ = frame.set(*vec, pixel.to_col());
    }
}


pub(crate) fn start(mut pb: QueryPixelBuffer) {
    // Set each pixel to a random color
    pb.frame().per_pixel(|_, _| Pixel::random());
}