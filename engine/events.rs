use array2d::Array2D;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_pixel_buffer::prelude::*;
use crate::{Grid, GridPos, PixelType, warn_on_err};

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
    if buttons.pressed(MouseButton::Left) {
        let window = q_window.single();
        let (camera, camera_transform) = q_camera.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            for buffer in q_buffer.iter() {
                let grid_position = (world_position /
                    buffer.size.pixel_size.as_vec2()
                    + (buffer.size.size.as_vec2() / 2.0)).floor();
                
                if grid_position.x < 0. || grid_position.y < 0. { return; }
                
                let mut grid_position = grid_position.as_uvec2();

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
    q_pixel_type: Query<&PixelType>,
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
                    if let Some(e) = grid.0.get_mut(vec.x as usize, vec.y as usize)
                        .and_then(|x| x.as_mut()) {
                        
                        if let Ok(pt) = q_pixel_type.get(*e) {
                            if (pt != pixel) {
                                cmd.entity(*e).despawn();
                                let entity = pixel.spawn(&mut cmd, vec);
                                grid.0.set(vec.x as usize, vec.y as usize, entity).unwrap();
                                warn_on_err(frame.set(*vec, pixel.to_col()));
                            }
                        }
                    } else {
                        
                        let entity = pixel.spawn(&mut cmd, vec);
                        grid.0.set(vec.x as usize, vec.y as usize, entity).unwrap();
                        frame.set(*vec, pixel.to_col()).unwrap()
                    }
                } else {
                    // No pixel given, meaning delete
                    if let Some(e) = gain(&mut grid.0, vec) {
                        
                        cmd.entity(e).despawn();
                        warn_on_err(frame.set(*vec, Color::NONE));
                    }
                }
            }
            
            // --- swap two pixels --- //
            PixelEventType::Swap(from_pt, to_pt) => {
                let from = get_e_and_pt(&mut grid, from_pt, &q_pixel_type);
                let to = get_e_and_pt(&mut grid, to_pt, &q_pixel_type);
                
                set_from_swap(&mut q_pixel_pos, &mut grid, &mut frame, from, *to_pt);
                set_from_swap(&mut q_pixel_pos, &mut grid, &mut frame, to, *from_pt);
            }
        }
    }
}

fn set_from_swap(
    q_pixel_pos: &mut Query<&mut GridPos>,
    grid: &mut ResMut<Grid>,
    frame: &mut Frame,
    swap: Option<(Entity, PixelType)>,
    set_to_point: UVec2,
) {
    if let Some((e, col)) = swap {
        warn_on_err(q_pixel_pos.get_mut(e).and_then(|mut e| { e.0 = set_to_point; Ok(()) }));
        grid.0.set(set_to_point.x as usize, set_to_point.y as usize, Some(e)).unwrap();
        warn_on_err(frame.set(set_to_point, col.to_col()));
    } else {
        grid.0.set(set_to_point.x as usize, set_to_point.y as usize, None).unwrap();
        warn_on_err(frame.set(set_to_point, Color::NONE));
    }
}

fn get_e_and_pt(
    grid: &mut Grid,
    pos: &UVec2,
    q_pixel_type: &Query<&PixelType>
) -> Option<(Entity, PixelType)> {
    if let Some(e) = gain(&mut grid.0, pos) {
        if let Some(pt) = warn_on_err(q_pixel_type.get(e)) {
            return Some((e, *pt));
        }
    }
    None
}

fn gain(
    grid: &mut Array2D<Option<Entity>>,
    pos: &UVec2,
) -> Option<Entity> {
    grid.get_mut(pos.x as usize, pos.y as usize).and_then(|x| x.take())
}