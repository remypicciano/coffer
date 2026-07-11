use eframe::egui;
use rfd::FileDialog;

use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Workflow {
    #[default]
    Home,
    Encrypt,
    Decrypt,
}

pub struct CofferApp {
    pub workflow: Workflow,

    pub encrypted_file: Option<PathBuf>,
    pub key_file: Option<PathBuf>,

    pub status: String,
    pub progress: f32,

    pub show_success: bool,
    pub show_error: bool,
    pub show_viewer: bool,

    pub decrypted_text: Option<String>,
}

impl Default for CofferApp {
    fn default() -> Self {
        Self {
            workflow: Workflow::Home,

            encrypted_file: None,
            key_file: None,

            status: "Ready".into(),
            progress: 0.0,

            show_success: false,
            show_error: false,
            show_viewer: false,

            decrypted_text: None,
        }
    }
}

impl CofferApp {
    pub fn open_encrypt_workflow(&mut self) {
        self.workflow = Workflow::Encrypt;
        self.reset_session();
    }

    pub fn open_decrypt_workflow(&mut self) {
        self.workflow = Workflow::Decrypt;
        self.reset_session();
    }

    pub fn return_home(&mut self) {
        self.workflow = Workflow::Home;
        self.reset_session();
    }

    fn reset_session(&mut self) {
        self.encrypted_file = None;
        self.key_file = None;

        self.status = "Ready".into();
        self.progress = 0.0;

        self.show_success = false;
        self.show_error = false;
        self.show_viewer = false;

        self.decrypted_text = None;
    }

    pub fn select_file(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Coffer encrypted files", &["coffer"])
            .pick_file()
        {
            self.encrypted_file = Some(path);
            self.status = "Encrypted file loaded".into();
        }
    }

    pub fn select_key(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Encryption key", &["bin"])
            .pick_file()
        {
            self.key_file = Some(path);
            self.status = "Key loaded".into();
        }
    }

    pub fn decrypt(&mut self) {
        if self.encrypted_file.is_none() || self.key_file.is_none() {
            self.status = "Missing file or key".into();

            self.show_error = true;

            return;
        }

        self.status = "Decrypting...".into();
        self.progress = 1.0;

        // Placeholder until the crypto layer is connected.
        self.decrypted_text = Some("Secret decrypted message.".into());

        self.status = "Decryption successful".into();

        self.show_success = true;
    }
}

impl eframe::App for CofferApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::ui::home::show(self, ctx);

        crate::ui::dialogs::show(self, ctx);
    }
}
