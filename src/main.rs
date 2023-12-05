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
            add_sand, change_type, log_positions
        ))
        .insert_resource(CurrentType(PixelType::Sand))
        .run()
}

// fn click_debug(mut ev_grid_click: EventReader<GridClickEvent>) {
//     for GridClickEvent(vec) in ev_grid_click.read() {
//         println!("Clicked on {:?}", vec);
//     }
// }

fn change_type(
    keyboard_input: Res<Input<KeyCode>>,
    mut current_type: ResMut<CurrentType>
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        current_type.0 = PixelType::Sand;
        println!("{:?}", current_type.0);
    }
    else if keyboard_input.just_pressed(KeyCode::Key2) {
        current_type.0 = PixelType::Stone;
        println!("{:?}", current_type.0);
    }
    else if keyboard_input.just_pressed(KeyCode::Key3) {
        current_type.0 = PixelType::Water;
        println!("{:?}", current_type.0);
    }
    else if keyboard_input.just_pressed(KeyCode::Key4) {
        current_type.0 = PixelType::Metal;
        println!("{:?}", current_type.0);
    }
    else if keyboard_input.just_pressed(KeyCode::Key5) {
        current_type.0 = PixelType::Dirt;
        println!("{:?}", current_type.0);
    }
    else if keyboard_input.just_pressed(KeyCode::Key6) {
        current_type.0 = PixelType::Lava;
        println!("{:?}", current_type.0);
    }
}

fn log_positions(
    keyboard_input: Res<Input<KeyCode>>,
    grid: Res<Grid>,
    mut commands: Commands
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        for (i, ent) in grid.0.as_column_major().iter().enumerate() {
            if let Some(ent) = ent {    
                println!("Entity {:?} found at Index {}, (x{}, y{}) ", ent, i, 
                         i % grid.0.column_len(), i / grid.0.column_len());
                commands.entity(*ent).log_components();
            }
        }
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