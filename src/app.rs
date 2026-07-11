use eframe::egui;
use rfd::FileDialog;

use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Workflow {
    Encrypt,

    #[default]
    Decrypt,

    Security,
    Settings,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OperationStage {
    #[default]
    SelectFiles,
    Review,
    Processing,
    Complete,
}

#[derive(Clone, Debug)]
pub struct SelectedFile {
    pub path: PathBuf,
    pub name: String,
    pub extension: String,
    pub size_bytes: u64,
}

impl SelectedFile {
    pub fn from_path(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown file")
            .to_string();

        let extension = path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("No extension")
            .to_string();

        let size_bytes = fs::metadata(&path)
            .map(|metadata| metadata.len())
            .unwrap_or(0);

        Self {
            path,
            name,
            extension,
            size_bytes,
        }
    }

    pub fn readable_size(&self) -> String {
        format_file_size(self.size_bytes)
    }
}

pub struct CofferApp {
    pub workflow: Workflow,
    pub stage: OperationStage,

    pub source_file: Option<SelectedFile>,
    pub encrypted_file: Option<SelectedFile>,
    pub key_file: Option<SelectedFile>,

    pub status: String,
    pub progress: f32,

    pub show_error: bool,
    pub error_message: String,

    pub show_viewer: bool,
    pub decrypted_text: Option<String>,
    pub main_window_size_before_viewer: Option<egui::Vec2>,
    pub viewer_was_open: bool,

    pub encryption_output: Option<PathBuf>,
    pub decryption_output: Option<PathBuf>,
}

impl Default for CofferApp {
    fn default() -> Self {
        Self {
            workflow: Workflow::Decrypt,
            stage: OperationStage::SelectFiles,

            source_file: None,
            encrypted_file: None,
            key_file: None,

            status: "Ready".into(),
            progress: 0.0,

            show_error: false,
            error_message: String::new(),

            show_viewer: false,
            decrypted_text: None,
            main_window_size_before_viewer: None,
            viewer_was_open: false,

            encryption_output: None,
            decryption_output: None,
        }
    }
}

impl CofferApp {
    pub fn navigate(&mut self, workflow: Workflow) {
        // Navigation intentionally preserves selected files.
        self.workflow = workflow;
        self.update_stage();
    }

    pub fn reset_session(&mut self) {
        self.stage = OperationStage::SelectFiles;

        self.source_file = None;
        self.encrypted_file = None;
        self.key_file = None;

        self.status = "Ready".into();
        self.progress = 0.0;

        self.show_error = false;
        self.error_message.clear();

        self.show_viewer = false;
        self.decrypted_text = None;

        self.encryption_output = None;
        self.decryption_output = None;
    }

    pub fn select_source_file(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            self.source_file = Some(SelectedFile::from_path(path));

            self.status = "Source file selected".into();

            self.update_stage();
        }
    }

    pub fn select_encrypted_file(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Coffer encrypted files", &["coffer"])
            .pick_file()
        {
            self.encrypted_file = Some(SelectedFile::from_path(path));

            self.status = "Encrypted file selected".into();

            self.update_stage();
        }
    }

    pub fn select_key(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Coffer keys", &["bin", "key"])
            .pick_file()
        {
            self.key_file = Some(SelectedFile::from_path(path));

            self.status = "Key selected".into();

            self.update_stage();
        }
    }

    pub fn clear_source_file(&mut self) {
        self.source_file = None;
        self.encryption_output = None;
        self.update_stage();
    }

    pub fn clear_encrypted_file(&mut self) {
        self.encrypted_file = None;
        self.decryption_output = None;
        self.update_stage();
    }

    pub fn clear_key_file(&mut self) {
        self.key_file = None;
        self.decryption_output = None;
        self.update_stage();
    }

    pub fn handle_dropped_files(&mut self, ctx: &egui::Context) {
        let dropped_files = ctx.input(|input| input.raw.dropped_files.clone());

        for dropped in dropped_files {
            let Some(path) = dropped.path else {
                continue;
            };

            match self.workflow {
                Workflow::Encrypt => {
                    self.source_file = Some(SelectedFile::from_path(path));

                    self.status = "Source file dropped".into();
                }

                Workflow::Decrypt => {
                    assign_decryption_drop(self, path);
                }

                Workflow::Security | Workflow::Settings => {}
            }
        }

        self.update_stage();
    }

    pub fn can_encrypt(&self) -> bool {
        self.source_file.is_some()
    }

    pub fn can_decrypt(&self) -> bool {
        self.encrypted_file.is_some() && self.key_file.is_some()
    }

    pub fn run_encrypt(&mut self) {
        if !self.can_encrypt() {
            self.show_error("Choose a file before encrypting.");

            return;
        }

        self.stage = OperationStage::Processing;
        self.status = "Encrypting file...".into();
        self.progress = 1.0;

        let output = self
            .source_file
            .as_ref()
            .map(|file| PathBuf::from(format!("{}.coffer", file.path.display())));

        self.encryption_output = output;

        self.status = "Encryption complete".into();

        self.stage = OperationStage::Complete;
    }

    pub fn run_decrypt(&mut self) {
        if !self.can_decrypt() {
            self.show_error("Choose both a .coffer file and its key before decrypting.");

            return;
        }

        self.stage = OperationStage::Processing;
        self.status = "Decrypting file...".into();
        self.progress = 1.0;

        // Placeholder until crypto.rs is connected.
        self.decrypted_text = Some("Secret decrypted message.".to_string());

        self.decryption_output = Some(PathBuf::from("Restored content"));

        self.status = "Decryption complete".into();

        self.stage = OperationStage::Complete;

        // Open the real secondary viewer immediately.
        self.show_viewer = true;
    }

    pub fn open_secure_viewer(&mut self) {
        if self.decrypted_text.is_some() {
            self.show_viewer = true;
        }
    }

    pub fn close_secure_viewer(&mut self) {
        self.show_viewer = false;
        self.viewer_was_open = false;
        self.decrypted_text = None;
    }

    pub fn show_error(&mut self, message: impl Into<String>) {
        self.error_message = message.into();
        self.show_error = true;
        self.status = "Action required".into();
    }

    fn update_stage(&mut self) {
        self.stage = match self.workflow {
            Workflow::Encrypt if self.source_file.is_some() => OperationStage::Review,

            Workflow::Decrypt if self.can_decrypt() => OperationStage::Review,

            Workflow::Encrypt | Workflow::Decrypt => OperationStage::SelectFiles,

            Workflow::Security | Workflow::Settings => OperationStage::SelectFiles,
        };

        self.progress = 0.0;
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let command = egui::Modifiers::COMMAND;

        if ctx.input_mut(|input| input.consume_key(command, egui::Key::E)) {
            self.navigate(Workflow::Encrypt);
        }

        if ctx.input_mut(|input| input.consume_key(command, egui::Key::D)) {
            self.navigate(Workflow::Decrypt);
        }
    }
}

impl eframe::App for CofferApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_shortcuts(ctx);
        self.handle_dropped_files(ctx);

        crate::ui::home::show(self, ctx);

        crate::ui::dialogs::show(self, ctx);

        crate::ui::secure_viewer::show(self, ctx);
    }
}

fn assign_decryption_drop(app: &mut CofferApp, path: PathBuf) {
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "coffer" => {
            app.encrypted_file = Some(SelectedFile::from_path(path));

            app.status = "Encrypted file dropped".into();
        }

        "bin" | "key" => {
            app.key_file = Some(SelectedFile::from_path(path));

            app.status = "Key file dropped".into();
        }

        _ => {
            app.show_error("Drop a .coffer file or a supported key file.");
        }
    }
}

fn format_file_size(size: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let size = size as f64;

    if size >= GB {
        format!("{:.2} GB", size / GB)
    } else if size >= MB {
        format!("{:.2} MB", size / MB)
    } else if size >= KB {
        format!("{:.1} KB", size / KB)
    } else {
        format!("{} bytes", size as u64)
    }
}
