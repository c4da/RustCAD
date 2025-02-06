use bevy_egui::egui;
use std::collections::VecDeque;
use bevy::prelude::*;
use crate::ai::{
    ai_client::AiClient, process_console_ai_command,
};


use tokio::runtime::Runtime;


#[derive(Resource)]
pub struct AsyncRuntime(pub Runtime);

// Component to store the async task
#[derive(Component)]
pub struct AsyncApiTask {
    input: String,
    client: AiClient,
}


#[derive(Default, Resource)]
pub struct OutputConsole {
    logs: VecDeque<String>,
    max_lines: usize,
    pub input_text: String,
}

impl OutputConsole {
    pub fn new(max_lines: usize) -> Self {
        Self {
            logs: VecDeque::with_capacity(max_lines),
            max_lines,
            input_text: String::new(),
        }
    }

    pub fn add_log(&mut self, message: impl Into<String>) {
        self.logs.push_back(message.into());
        while self.logs.len() > self.max_lines {
            self.logs.pop_front();
        }
    }

    pub fn show(
        &mut self,
        mut commands: Commands,
        ai_client: Res<AiClient>,
        ui: &mut egui::Ui) {
        // ui.heading("Console Input");
        let input_field = ui.add(egui::TextEdit::singleline(&mut self.input_text));
        if input_field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            println!("Sending input: {}", self.input_text);
            let task = AsyncApiTask {
                input: self.input_text.clone(),
                client: ai_client.clone(),
            };

            // Spawn an entity to track the task
            commands.spawn(task);
            self.logs.push_back(std::mem::take(&mut self.input_text));
        }

        // ui.heading("Console Output");
        
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for log in &self.logs {
                    ui.label(log);
                }
            });
    }

    pub fn clear(&mut self) {
        self.logs.clear();
    }
}

pub fn console_ui_system(mut commands: Commands, aiClient: Res<AiClient>, mut console: ResMut<OutputConsole>, mut egui_contexts: bevy_egui::EguiContexts) {
    // Renders a bottom panel for the console
    egui::Window::new("Console")
        .resizable(true)
        .movable(true)
        .collapsible(true)
        .fixed_size((400.0, 300.0))                // Example fixed size
        // .anchor(egui::Align2::CENTER_BOTTOM, [0.0, 0.0]) // Center it on the screen
        .show(egui_contexts.ctx_mut(), |ui| {
            console.show(commands, aiClient, ui);
        });

    // egui::TopBottomPanel::bottom("console_panel")
    //     .show(egui_contexts.ctx_mut(), |ui| {
    //         console.show(commands, aiClient, ui);
    //     });
}

pub fn handle_api_response(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tasks: Query<(Entity, &AsyncApiTask)>,
    runtime: Res<AsyncRuntime>,
) {
    for (entity, task) in tasks.iter() {
        let result = runtime.0.block_on(async {
            task.client.call_llm_api(&task.input).await
        });

        match result {
            Ok(response) => {
                process_console_ai_command(&response, &mut commands, &mut meshes, &mut materials);
            }
            Err(e) => {
                println!("Error calling LLM API: {:?}", e);
            }
        }

        commands.entity(entity).despawn();
    }
}