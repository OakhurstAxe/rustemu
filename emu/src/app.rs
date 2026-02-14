

use std::path::PathBuf;

use vcs::vcs_sdlmain::vcs::VcsSdlMain;
use egui_file_dialog::FileDialog;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.

pub struct TemplateApp {
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
    vcs: VcsSdlMain,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            picked_file: None,
            vcs: VcsSdlMain::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new() -> Self {
            Default::default()
    }
}

impl eframe::App for TemplateApp {

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Update the dialog
        self.file_dialog.update(ctx);
        let path= self.file_dialog.take_picked();
        if let Some(x) = path {
            self.vcs.vcs_sdl_main(&x.into_os_string().into_string().unwrap());
        };

        // Check if the user picked a file.
        if let Some(path) = self.file_dialog.take_picked() {
            self.picked_file = Some(path.to_path_buf());
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Atari VCS Rom").clicked() {
                            self.file_dialog.pick_file();
                        }
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
        });
    }
}

