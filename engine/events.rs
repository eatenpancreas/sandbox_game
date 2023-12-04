use array2d::Array2D;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_pixel_buffer::prelude::*;
use crate::{Grid, GridPos, PixelType};

#[derive(Event, Debug)]
pub struct GridClickEvent(pub UVec2);
#[derive(Event, Debug)]
pub struct SetPixelEvent(pub PixelEventType);
pub type PixelTypeInstance = (UVec2, Option<PixelType>);
#[derive(Debug)]
pub enum PixelEventType {
    Set(PixelTypeInstance),
    Swap(UVec2, UVec2)
}

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

                let y = buffer.size.size.y as i32 - grid_position.y as i32 - 1;
                if y >= 0 {
                    grid_position.y = y as u32;
                    if grid_position.x < buffer.size.size.x
                        && grid_position.y < buffer.size.size.y {
                        ev_grid_click.send(GridClickEvent(grid_position));
                    }
                }
            }
        }
    }
}

pub(crate) fn set_pixel(
    mut r_pixel: EventReader<SetPixelEvent>,
    mut pb: QueryPixelBuffer,
    mut q_pixel_type: Query<(Entity, &PixelType)>,
    mut q_pixel_pos: Query<&mut GridPos>,
    mut grid: ResMut<Grid>,
    mut cmd: Commands
) {
    let r = r_pixel.read();
    if r.len() <= 0 { return; }
    let mut frame = pb.frame();

    for SetPixelEvent(e_type) in r {
        match e_type {
            // --- set pixel at point --- //
            PixelEventType::Set((vec, pix)) => {
                if let Some(pixel) = pix {
                    // This square now has a pixel
                    if let Some(entity) = grid.0.get_mut(vec.x as usize, vec.y as usize)
                        .and_then(|x| x.as_mut()) {
                        
                        let _ = frame.set(*vec, pixel.to_col());
                    } else {
                        let entity = pixel.spawn(&mut cmd, vec);
                        let _ = grid.0.set(vec.x as usize, vec.y as usize, entity);
                        let _ = frame.set(*vec, pixel.to_col());
                    }
                } else {
                    // No pixel given, meaning delete
                    if let Some(e) = gain(&mut grid.0, vec) {
                        
                        cmd.entity(e).despawn();
                        let _ = frame.set(*vec, Color::NONE);
                    }
                }
            }
            
            // --- swap two pixels --- //
            PixelEventType::Swap(from, to) => {
                let (from_e, from_pix) = gain(&mut grid.0, from).and_then(
                    |e| q_pixel_type.get(e).ok()).unzip();
                let (to_e, to_pix)= gain(&mut grid.0, to).and_then(
                    |e| q_pixel_type.get(e).ok()).unzip();
                
                if let Some(from_e) = from_e {
                    let _ = q_pixel_pos.get_mut(from_e).and_then(|mut e| { e.0 = *to; Ok(()) });
                }
                if let Some(to_e) = to_e {
                    let _ = q_pixel_pos.get_mut(to_e).and_then(|mut e| { e.0 = *from; Ok(()) });
                }
                
                let _ = grid.0.set(to.x as usize, to.y as usize, from_e);
                let _ = frame.set(*to, from_pix.and_then(|pix| Some(pix.to_col())).unwrap_or(Color::NONE));
                let _ = grid.0.set(from.x as usize, from.y as usize, to_e);
                let _ = frame.set(*from, to_pix.and_then(|pix| Some(pix.to_col())).unwrap_or(Color::NONE));
            }
        }
    }
}

fn gain(
    grid: &mut Array2D<Option<Entity>>,
    pos: &UVec2,
) -> Option<Entity> {
    grid.get_mut(pos.x as usize, pos.y as usize).and_then(|x| x.take())
}