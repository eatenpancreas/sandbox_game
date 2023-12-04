
use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;
use sandbox_engine::*;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, PixelBufferPlugin,
            SandboxPlugin {
                size: UVec2::new(64, 64),
                pixel_size: UVec2::new(8, 8),
            }, SandboxPhysicsPlugin
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
        set_pixel.send(SetPixelEvent(
            PixelEventType::Set((*vec, Some(PixelType::Sand)))
        ))
    }
}