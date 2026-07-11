use eframe::egui;
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Workflow {
    Protect,
    #[default]
    Open,
    Security,
    Settings,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProtectStage {
    #[default]
    SelectFile,
    Review,
    Processing,
    Complete,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProtectKeySource {
    #[default]
    GenerateNew,
    Existing,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OpenStage {
    #[default]
    SelectContainer,
    SelectKey,
    Review,
    Processing,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoticeKind {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Debug)]
pub struct Notice {
    pub kind: NoticeKind,
    pub message: String,
}

impl Notice {
    fn new(kind: NoticeKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
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
    pub protect_stage: ProtectStage,
    pub open_stage: OpenStage,
    pub protect_key_source: ProtectKeySource,
    pub scroll_to_protect_key: bool,
    pub source_file: Option<SelectedFile>,
    pub protect_key_file: Option<SelectedFile>,
    pub encrypted_file: Option<SelectedFile>,
    pub key_file: Option<SelectedFile>,
    pub notice: Notice,
    pub progress: f32,
    pub show_error: bool,
    pub error_message: String,
    pub show_viewer: bool,
    pub decrypted_text: Option<String>,
    pub main_window_size_before_viewer: Option<egui::Vec2>,
    pub viewer_was_open: bool,
    pub encryption_output: Option<PathBuf>,
    pub key_output: Option<PathBuf>,
    pub decryption_output: Option<PathBuf>,
    pub ask_for_output_location: bool,
    pub confirm_before_replace: bool,
    pub offer_text_preview: bool,
    pub clear_recent_locations: bool,
}

impl Default for CofferApp {
    fn default() -> Self {
        Self {
            workflow: Workflow::Open,
            protect_stage: ProtectStage::SelectFile,
            open_stage: OpenStage::SelectContainer,
            protect_key_source: ProtectKeySource::GenerateNew,
            scroll_to_protect_key: false,
            source_file: None,
            protect_key_file: None,
            encrypted_file: None,
            key_file: None,
            notice: Notice::new(NoticeKind::Info, "Ready"),
            progress: 0.0,
            show_error: false,
            error_message: String::new(),
            show_viewer: false,
            decrypted_text: None,
            main_window_size_before_viewer: None,
            viewer_was_open: false,
            encryption_output: None,
            key_output: None,
            decryption_output: None,
            ask_for_output_location: true,
            confirm_before_replace: true,
            offer_text_preview: true,
            clear_recent_locations: true,
        }
    }
}

impl CofferApp {
    pub fn navigate(&mut self, workflow: Workflow) {
        self.workflow = workflow;
        let kind = match workflow {
            Workflow::Security | Workflow::Settings => NoticeKind::Success,
            Workflow::Protect | Workflow::Open => NoticeKind::Info,
        };
        self.set_notice(
            kind,
            match workflow {
                Workflow::Protect => "Choose a file to protect",
                Workflow::Open => "Choose a protected file",
                Workflow::Security | Workflow::Settings => "Local • Offline • Private",
            },
        );
    }

    pub fn reset_protect(&mut self) {
        self.protect_stage = ProtectStage::SelectFile;
        self.source_file = None;
        self.protect_key_source = ProtectKeySource::GenerateNew;
        self.scroll_to_protect_key = false;
        self.protect_key_file = None;
        self.encryption_output = None;
        self.key_output = None;
        self.progress = 0.0;
        self.set_notice(NoticeKind::Info, "Choose a file to protect");
    }

    pub fn reset_open(&mut self) {
        self.open_stage = OpenStage::SelectContainer;
        self.encrypted_file = None;
        self.key_file = None;
        self.decryption_output = None;
        self.decrypted_text = None;
        self.progress = 0.0;
        self.set_notice(NoticeKind::Info, "Choose a protected file");
    }

    pub fn select_source_file(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            self.source_file = Some(SelectedFile::from_path(path));
            self.protect_stage = ProtectStage::SelectFile;
            self.set_notice(NoticeKind::Info, "File selected — continue to review");
        }
    }

    pub fn select_encrypted_file(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Coffer protected files", &["coffer"])
            .pick_file()
        {
            self.encrypted_file = Some(SelectedFile::from_path(path));
            self.open_stage = OpenStage::SelectKey;
            self.set_notice(NoticeKind::Info, "Now choose the matching key");
        }
    }

    pub fn select_protect_key(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Coffer keys", &["key"])
            .pick_file()
        {
            self.protect_key_file = Some(SelectedFile::from_path(path));
            self.set_notice(
                NoticeKind::Info,
                "Existing key selected — continue to review",
            );
        }
    }

    pub fn set_protect_key_source(&mut self, source: ProtectKeySource) {
        self.protect_key_source = source;
        if source == ProtectKeySource::GenerateNew {
            self.protect_key_file = None;
            self.scroll_to_protect_key = false;
            self.set_notice(NoticeKind::Info, "Coffer will create a new random key");
        } else {
            self.scroll_to_protect_key = true;
            self.set_notice(NoticeKind::Warning, "Choose an existing Coffer key");
        }
    }

    pub fn clear_protect_key(&mut self) {
        self.protect_key_file = None;
        self.set_notice(NoticeKind::Warning, "Choose an existing Coffer key");
    }

    pub fn select_key(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Coffer keys", &["key", "bin"])
            .pick_file()
        {
            self.key_file = Some(SelectedFile::from_path(path));
            self.open_stage = OpenStage::SelectKey;
            self.set_notice(NoticeKind::Info, "Key selected — continue to review");
        }
    }

    pub fn clear_source_file(&mut self) {
        self.source_file = None;
        self.encryption_output = None;
        self.key_output = None;
        self.protect_stage = ProtectStage::SelectFile;
        self.set_notice(NoticeKind::Info, "Choose a file to protect");
    }

    pub fn clear_encrypted_file(&mut self) {
        self.encrypted_file = None;
        self.decryption_output = None;
        self.open_stage = OpenStage::SelectContainer;
        self.set_notice(NoticeKind::Info, "Choose a protected file");
    }

    pub fn clear_key_file(&mut self) {
        self.key_file = None;
        self.decryption_output = None;
        self.open_stage = if self.encrypted_file.is_some() {
            OpenStage::SelectKey
        } else {
            OpenStage::SelectContainer
        };
        self.set_notice(NoticeKind::Info, "Choose the matching key");
    }

    pub fn review_protect(&mut self) {
        if self.can_protect() {
            self.protect_stage = ProtectStage::Review;
            self.set_notice(
                NoticeKind::Info,
                "Review where your protected files will be saved",
            );
        }
    }

    pub fn review_open(&mut self) {
        if self.can_open() {
            self.open_stage = OpenStage::Review;
            self.set_notice(NoticeKind::Info, "Review before restoring the file");
        }
    }

    pub fn back(&mut self) {
        match self.workflow {
            Workflow::Protect => self.protect_stage = ProtectStage::SelectFile,
            Workflow::Open => {
                self.open_stage = if self.encrypted_file.is_some() {
                    OpenStage::SelectKey
                } else {
                    OpenStage::SelectContainer
                }
            }
            Workflow::Security | Workflow::Settings => {}
        }
        self.set_notice(NoticeKind::Info, "Selections preserved");
    }

    pub fn can_protect(&self) -> bool {
        self.source_file.is_some()
            && (self.protect_key_source == ProtectKeySource::GenerateNew
                || self.protect_key_file.is_some())
    }

    pub fn can_open(&self) -> bool {
        self.encrypted_file.is_some() && self.key_file.is_some()
    }

    pub fn run_protect(&mut self) {
        if !self.can_protect() {
            self.show_error("Choose a file before continuing.");
            return;
        }

        self.protect_stage = ProtectStage::Processing;
        self.progress = 1.0;
        if let Some(file) = self.source_file.as_ref() {
            self.encryption_output = Some(PathBuf::from(format!("{}.coffer", file.path.display())));
            self.key_output = if self.protect_key_source == ProtectKeySource::GenerateNew {
                Some(PathBuf::from(format!("{}.coffer.key", file.path.display())))
            } else {
                None
            };
        }
        self.protect_stage = ProtectStage::Complete;
        self.set_notice(
            NoticeKind::Warning,
            "Prototype complete — no files were written",
        );
    }

    pub fn run_open(&mut self) {
        if !self.can_open() {
            self.show_error("Choose both the protected file and its matching key.");
            return;
        }

        self.open_stage = OpenStage::Processing;
        self.progress = 1.0;
        self.decrypted_text = Some("Secret decrypted message.".to_string());
        self.decryption_output = Some(PathBuf::from("Restored content"));
        self.open_stage = OpenStage::Complete;
        self.set_notice(
            NoticeKind::Warning,
            "Prototype complete — no file was restored",
        );
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
        self.set_notice(NoticeKind::Error, "Action required");
    }

    pub fn handle_dropped_files(&mut self, ctx: &egui::Context) {
        let dropped_files = ctx.input(|input| input.raw.dropped_files.clone());
        for dropped in dropped_files {
            let Some(path) = dropped.path else { continue };
            match self.workflow {
                Workflow::Protect => {
                    let is_key = path
                        .extension()
                        .and_then(|extension| extension.to_str())
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("key"));
                    if self.protect_key_source == ProtectKeySource::Existing && is_key {
                        self.protect_key_file = Some(SelectedFile::from_path(path));
                        self.set_notice(NoticeKind::Info, "Existing key selected");
                    } else {
                        self.source_file = Some(SelectedFile::from_path(path));
                        self.protect_stage = ProtectStage::SelectFile;
                        self.set_notice(NoticeKind::Info, "File selected — continue to review");
                    }
                }
                Workflow::Open => assign_open_drop(self, path),
                Workflow::Security | Workflow::Settings => {}
            }
        }
    }

    fn set_notice(&mut self, kind: NoticeKind, message: impl Into<String>) {
        self.notice = Notice::new(kind, message);
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let command = egui::Modifiers::COMMAND;
        if ctx.input_mut(|input| input.consume_key(command, egui::Key::E)) {
            self.navigate(Workflow::Protect);
        }
        if ctx.input_mut(|input| input.consume_key(command, egui::Key::D)) {
            self.navigate(Workflow::Open);
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

fn assign_open_drop(app: &mut CofferApp, path: PathBuf) {
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "coffer" => {
            app.encrypted_file = Some(SelectedFile::from_path(path));
            app.open_stage = OpenStage::SelectKey;
            app.set_notice(NoticeKind::Info, "Protected file selected");
        }
        "bin" | "key" => {
            app.key_file = Some(SelectedFile::from_path(path));
            app.open_stage = if app.encrypted_file.is_some() {
                OpenStage::SelectKey
            } else {
                OpenStage::SelectContainer
            };
            app.set_notice(NoticeKind::Info, "Key selected");
        }
        _ => app.show_error("Drop a .coffer file or a supported .key file."),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn selected(name: &str) -> SelectedFile {
        SelectedFile {
            path: PathBuf::from(name),
            name: name.to_string(),
            extension: name
                .rsplit_once('.')
                .map(|(_, ext)| ext)
                .unwrap_or("")
                .to_string(),
            size_bytes: 42,
        }
    }

    #[test]
    fn navigation_preserves_selected_files() {
        let mut app = CofferApp {
            source_file: Some(selected("notes.txt")),
            ..Default::default()
        };
        app.navigate(Workflow::Settings);
        app.navigate(Workflow::Protect);
        assert!(app.source_file.is_some());
    }

    #[test]
    fn protect_requires_explicit_review_transition() {
        let mut app = CofferApp {
            source_file: Some(selected("notes.txt")),
            ..Default::default()
        };
        assert_eq!(app.protect_stage, ProtectStage::SelectFile);
        app.review_protect();
        assert_eq!(app.protect_stage, ProtectStage::Review);
    }

    #[test]
    fn open_requires_both_files_before_review() {
        let mut app = CofferApp {
            encrypted_file: Some(selected("notes.coffer")),
            ..Default::default()
        };
        app.review_open();
        assert_ne!(app.open_stage, OpenStage::Review);
        app.key_file = Some(selected("notes.key"));
        app.review_open();
        assert_eq!(app.open_stage, OpenStage::Review);
    }

    #[test]
    fn notices_have_explicit_severity() {
        let notice = Notice::new(NoticeKind::Success, "Complete");
        assert_eq!(notice.kind, NoticeKind::Success);
    }

    #[test]
    fn existing_key_must_be_selected_before_protect_review() {
        let mut app = CofferApp {
            source_file: Some(selected("notes.txt")),
            protect_key_source: ProtectKeySource::Existing,
            ..Default::default()
        };
        assert!(!app.can_protect());
        app.review_protect();
        assert_eq!(app.protect_stage, ProtectStage::SelectFile);

        app.protect_key_file = Some(selected("shared.key"));
        assert!(app.can_protect());
        app.review_protect();
        assert_eq!(app.protect_stage, ProtectStage::Review);
    }

    #[test]
    fn existing_key_mode_does_not_plan_a_new_key_file() {
        let mut app = CofferApp {
            source_file: Some(selected("notes.txt")),
            protect_key_source: ProtectKeySource::Existing,
            protect_key_file: Some(selected("shared.key")),
            ..Default::default()
        };
        app.run_protect();
        assert!(app.encryption_output.is_some());
        assert!(app.key_output.is_none());
    }

    #[test]
    fn choosing_existing_key_requests_one_scroll() {
        let mut app = CofferApp::default();
        app.set_protect_key_source(ProtectKeySource::Existing);
        assert!(app.scroll_to_protect_key);

        app.set_protect_key_source(ProtectKeySource::GenerateNew);
        assert!(!app.scroll_to_protect_key);
    }
}
