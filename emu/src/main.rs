

use std::time::Duration;

use egui_elm::prelude::*;

use vcs::vcs_console::vcs::{ VcsConsole, Message };

fn init(ctx: &egui::Context) -> (VcsConsole, Command<Message>) {
    (VcsConsole::new(ctx), Command::none())
}

fn update(vcs_console: &mut VcsConsole, message: Message) -> Command<Message> {

    match message {
        Message::Tick => vcs_console.start_next_frame()
    }
    Command::none()
}

fn view(vcs_console: &VcsConsole, _ctx: &egui::Context, _ui_ctx: &ViewContext<Message>) {
    vcs_console.update();

    vcs_console.input_check();
    
    //egui::TopBottomPanel::top("top").show(&ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's
        //ui.heading("eframe template");
    //});
}

fn subscription(_vcs_console: &VcsConsole) -> Subscription<Message> {
    egui_elm::subscription::Subscription::interval(Duration::from_millis(17), Message::Tick)
}

pub fn main() -> eframe::Result<()> {

    let program: Program<VcsConsole, Message> = Program::new(init, update, view, subscription);
    let native_options = eframe::NativeOptions {
            // Configure your window and OpenGL settings here
            renderer: eframe::Renderer::Glow, // Explicitly choose Glow (OpenGL)
            ..Default::default()
    };
    egui_elm::app::run_with_native_options(program, "Atari VCS", native_options)
    //egui_elm::app::run(program, "Atari VCS")
}

