
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

fn change_type(
    keyboard_input: Res<Input<KeyCode>>,
    mut current_type: ResMut<CurrentType>
) {
    for pr in keyboard_input.get_pressed() {
        if let Some(p_type) = match pr {
            KeyCode::Key1 => Some(PixelType::Sand),
            KeyCode::Key2 => Some(PixelType::Stone),
            KeyCode::Key3 => Some(PixelType::Water),
            KeyCode::Key4 => Some(PixelType::Metal),
            KeyCode::Key5 => Some(PixelType::Dirt),
            KeyCode::Key6 => Some(PixelType::Lava),
            _ => None
        } {
            current_type.0 = p_type;
            println!("{:?}", p_type);
        }
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
    current_type: Res<CurrentType>
) {
    for GridClickEvent(vec) in ev_grid_click.read() {
        set_pixel.send(SetPixelEvent(
            PixelEventType::Set((*vec, Some(current_type.0)))
        ))
    }
}