use bevy::asset::Assets;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Commands, Mesh, ResMut};
use serde_json::Value;
use crate::ai::json_parser;
use crate::ai::json_parser::LlmCubeCommand;
use crate::part;
use crate::part::primitives;


pub fn process_console_ai_command(
    llm_response: &String,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    let full_command: Value = serde_json::from_str(llm_response).unwrap();
    let command: &str = Box::leak(full_command["command"].as_str().unwrap().to_string().into_boxed_str());
    match command {
        "create cube" => {
            let command = json_parser::parse_cube_command(&llm_response);
            create_cube_from_command(&command, commands, meshes, materials);
            println!("Created cube");
        }
        "create cubes" => {
            let command = json_parser::parse_cubes_command(&llm_response);
            create_cubes_from_command(&command, commands, meshes, materials);
            println!("Cubes created");
        }
        "help" => {
            println!("Available commands:");
            println!("  create cube - Creates a cube");
            println!("  help        - Shows this help message");
        }
        "" => {
            println!("Type 'help' for available commands");
        }
        _ => {
            println!("Unknown command: '{}'. Type 'help' for available commands.", command);
        }
    }
}

fn create_cube_from_command(command: &LlmCubeCommand, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) {
    let mut points = primitives::CubePoints::get_points();

    for point in &mut points {
        *point += command.get_vector_from_origin();
    }

    part::create_3d_object_system(commands, meshes, materials, points);
}

fn create_cubes_from_command(command: &Vec<LlmCubeCommand>, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) {
    println!("Creating cubes");
    let points = primitives::CubePoints::get_points();

    command.iter().for_each(|cube_command| {
        let mut cube_pos = points.clone();
        for point in &mut cube_pos {
            *point += cube_command.get_vector_from_origin();
        }
        part::create_3d_object_system(commands, meshes, materials, cube_pos);
        print!("Created cube at: {:?}", cube_command.get_vector_from_origin());
    });
}