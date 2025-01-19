use std::borrow::Cow;
use bevy::math::Vec3;
use serde::Deserialize;
use serde_json::*;
use std::fmt;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct LlmCubeCommand {
    command: &'static str,
    parameters: (f32, f32, f32, f32, f32, f32),
}

impl fmt::Display for LlmCubeCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Command: {}, Parameters: {:?}", self.command, self.parameters)
    }
}

impl LlmCubeCommand {

    pub fn new(command: &'static str, parameters: (f32, f32, f32, f32, f32, f32)) -> Self {
        LlmCubeCommand {
            command,
            parameters,
        }
    }

    pub fn get_command(&self) -> &str {
        &self.command
    }

    pub fn get_parameters(&self) -> &(f32, f32, f32, f32, f32, f32) {
        &self.parameters
    }

    pub fn get_x(&self) -> f32 {
        self.parameters.0
    }

    pub fn get_y(&self) -> f32 {
        self.parameters.1
    }

    pub fn get_z(&self) -> f32 {
        self.parameters.2
    }

    pub fn get_width(&self) -> f32 {
        self.parameters.3
    }

    pub fn get_height(&self) -> f32 {
        self.parameters.4
    }

    pub fn get_depth(&self) -> f32 {
        self.parameters.5
    }
    //vector from origin to bottom left corner of the front face of the cube
    pub fn get_vector_from_origin(&self) -> Vec3 {
        Vec3::new(self.get_x(), self.get_y(), self.get_z())
    }

    pub fn get_dimensions(&self) -> Vec3 {
        Vec3::new(self.get_width(), self.get_height(), self.get_depth())
    }
}

pub fn parse_llm_output(json_str: &str) -> LlmCubeCommand {
    println!("Parsing JSON: {}", json_str);
    let full_command: Value = serde_json::from_str(json_str).unwrap();
    let command = Box::leak(full_command["command"].as_str().unwrap().to_string().into_boxed_str());
    let parameters = (
        full_command["parameters"]["x"].as_f64().unwrap() as f32,
        full_command["parameters"]["y"].as_f64().unwrap() as f32,
        full_command["parameters"]["z"].as_f64().unwrap() as f32,
        full_command["parameters"]["width"].as_f64().unwrap() as f32,
        full_command["parameters"]["height"].as_f64().unwrap() as f32,
        full_command["parameters"]["depth"].as_f64().unwrap() as f32,
    );
    LlmCubeCommand::new(command, parameters)
}

