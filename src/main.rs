
use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;
use sandbox_engine::{GridClickEvent, PixelType, SandboxPlugin, SetPixelEvent};


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, PixelBufferPlugin, SandboxPlugin
        )).add_systems(Update, (
            click_debug, add_sand,
        ))
        .run()
}

fn click_debug(mut ev_grid_click: EventReader<GridClickEvent>) {
    for GridClickEvent(vec) in ev_grid_click.read() {
        println!("Clicked on {:?}", vec);
    }
}

fn add_sand(
    mut ev_grid_click: EventReader<GridClickEvent>,
    mut set_pixel: EventWriter<SetPixelEvent>,
) {
    for GridClickEvent(vec) in ev_grid_click.read() {
        set_pixel.send(SetPixelEvent(*vec, PixelType::Sand))
    }
}