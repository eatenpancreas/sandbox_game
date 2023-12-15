
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_pixel_buffer::prelude::*;
use sandbox_engine::*;

#[derive(Resource, Copy, Clone)]
struct CurrentType(Option<PixelType>);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, PixelBufferPlugin, EguiPlugin,
            SandboxPlugin {
                size: UVec2::new(256, 256),
                pixel_size: UVec2::new(2, 2),
            }, SandboxPhysicsPlugin
        )).add_systems(Update, (
            add_sand, change_type, log_positions, pixel_type_ui
        ))
        .insert_resource(CurrentType(Some(PixelType::Sand)))
        .run()
}

fn pixel_type_ui(
    mut contexts: EguiContexts
) {
    let ctx = contexts.ctx_mut();
    let mut new_type = None;
    
    egui::SidePanel::left("left-panel")
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            if ui.button("Sand").clicked() { new_type = Some(Some(PixelType::Sand));}
        });
}

fn change_type(
    keyboard_input: Res<Input<KeyCode>>,
    mut current_type: ResMut<CurrentType>
) {
    for pr in keyboard_input.get_pressed() {
        let p_type = match pr {
            KeyCode::Key1 => Some(PixelType::Sand),
            KeyCode::Key2 => Some(PixelType::Stone),
            KeyCode::Key3 => Some(PixelType::Water),
            KeyCode::Key4 => Some(PixelType::Metal),
            KeyCode::Key5 => Some(PixelType::Dirt),
            KeyCode::Key6 => Some(PixelType::Lava),
            KeyCode::Key0 => None,
            _ => break
        };
        current_type.0 = p_type;
    }
}

fn log_positions(
    keyboard_input: Res<Input<KeyCode>>,
    grid: Res<Grid>,
    mut commands: Commands
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        println!("Getting entities");
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
        let mut vec = *vec;
        set_pixel.send(SetPixelEvent(
            PixelEventType::Set((vec, current_type.0))
        ));
        if vec.x > 0 {
            vec.x -= 1;
            set_pixel.send(SetPixelEvent(
                PixelEventType::Set((vec, current_type.0))
            ));
        }
        if vec.y > 0 {
            vec.y -= 1;
            set_pixel.send(SetPixelEvent(
                PixelEventType::Set((vec, current_type.0))
            ));
            vec.x += 1;
            set_pixel.send(SetPixelEvent(
                PixelEventType::Set((vec, current_type.0))
            ));
        }
    }
}