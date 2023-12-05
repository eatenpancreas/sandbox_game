use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;
use sandbox_engine::*;

#[derive(Resource, Copy, Clone)]
struct CurrentType(PixelType);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, PixelBufferPlugin,
            SandboxPlugin {
                size: UVec2::new(256, 256),
                pixel_size: UVec2::new(2, 2),
            }, SandboxPhysicsPlugin
        )).add_systems(Update, (
            click_debug, add_sand, change_type
        ))
        .insert_resource(CurrentType(PixelType::Sand))
        .run()
}

fn click_debug(mut ev_grid_click: EventReader<GridClickEvent>) {
    for GridClickEvent(vec) in ev_grid_click.read() {
        println!("Clicked on {:?}", vec);
    }
}

fn change_type(
    mut keyboard_input: Res<Input<KeyCode>>,
    mut current_type: ResMut<CurrentType>
) {
    if keyboard_input.just_pressed(KeyCode::U) {
        current_type.0 = PixelType::Sand;
        println!("{:?}", current_type.0)
    }
    if keyboard_input.just_pressed(KeyCode::Y) {
        current_type.0 = PixelType::Stone
    }
}

fn add_sand(
    mut ev_grid_click: EventReader<GridClickEvent>,
    mut set_pixel: EventWriter<SetPixelEvent>,
    mut current_type: Res<CurrentType>
) {
    for GridClickEvent(vec) in ev_grid_click.read() {
        set_pixel.send(SetPixelEvent(
            PixelEventType::Set((*vec, Some(current_type.0)))
        ))
    }
}