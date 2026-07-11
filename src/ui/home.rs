use eframe::egui;

use crate::app::{CofferApp, OpenStage, ProtectKeySource, ProtectStage, Workflow};
use crate::ui::{theme, widgets};

pub fn show(app: &mut CofferApp, ctx: &egui::Context) {
    let window_width = ctx.input(|input| input.content_rect().width());
    let compact = window_width < 900.0;
    let sidebar_width = if compact { 196.0 } else { 240.0 };
    let page_margin = if compact { 18 } else { 26 };

    egui::SidePanel::left("coffer_sidebar")
        .exact_width(sidebar_width)
        .resizable(false)
        .frame(
            egui::Frame::new()
                .fill(theme::SURFACE)
                .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
                .inner_margin(16.0),
        )
        .show(ctx, |ui| sidebar(app, ui, compact));

    egui::TopBottomPanel::bottom("coffer_action_bar")
        .resizable(false)
        .frame(
            egui::Frame::new()
                .fill(theme::SURFACE)
                .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
                .inner_margin(egui::Margin::symmetric(if compact { 16 } else { 24 }, 14)),
        )
        .show(ctx, |ui| action_bar(app, ui));

    egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(theme::BACKGROUND)
                .inner_margin(page_margin),
        )
        .show(ctx, |ui| content(app, ui));
}

fn sidebar(app: &mut CofferApp, ui: &mut egui::Ui, compact: bool) {
    ui.vertical_centered(|ui| {
        ui.add(
            egui::Image::new(egui::include_image!("../../assets/Pastelito_img.webp"))
                .fit_to_exact_size(egui::Vec2::splat(if compact { 44.0 } else { 58.0 }))
                .corner_radius(14.0),
        );
        ui.add_space(9.0);
        ui.heading(
            egui::RichText::new("Coffer")
                .size(if compact { 22.0 } else { 25.0 })
                .strong()
                .color(theme::TEXT_PRIMARY),
        );
        ui.label(
            egui::RichText::new("Private file protection")
                .small()
                .color(theme::TEXT_SECONDARY),
        );
    });

    ui.add_space(if compact { 20.0 } else { 28.0 });
    nav_item(
        app,
        ui,
        Workflow::Protect,
        "▣  Protect",
        "Create a protected copy",
    );
    nav_item(
        app,
        ui,
        Workflow::Open,
        "□  Open",
        "Restore a protected file",
    );
    ui.add_space(14.0);
    ui.separator();
    ui.add_space(14.0);
    nav_item(
        app,
        ui,
        Workflow::Security,
        "◇  Security",
        "How Coffer protects you",
    );
    nav_item(
        app,
        ui,
        Workflow::Settings,
        "⚙  Settings",
        "Application preferences",
    );

    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        ui.label(
            egui::RichText::new("Local • Offline • Private")
                .small()
                .color(theme::TEXT_MUTED),
        );
        ui.add_space(10.0);
        ui.horizontal_centered(|ui| widgets::status_pill(ui, &app.notice));
    });
}

fn nav_item(app: &mut CofferApp, ui: &mut egui::Ui, workflow: Workflow, label: &str, help: &str) {
    if widgets::nav_button(ui, label, app.workflow == workflow)
        .on_hover_text(help)
        .clicked()
    {
        app.navigate(workflow);
    }
}

fn content(app: &mut CofferApp, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let available = ui.available_width();

            if available >= 1_060.0 {
                let layout_width = available.min(1_240.0);
                let side_space = ((available - layout_width) * 0.5).max(0.0);
                ui.horizontal_top(|ui| {
                    ui.add_space(side_space);
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(layout_width - 316.0, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| workflow_content(app, ui),
                    );
                    ui.add_space(24.0);
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(292.0, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| context_panel(app, ui),
                    );
                });
            } else {
                let content_width = available.min(860.0);
                let side_space = ((available - content_width) * 0.5).max(0.0);
                ui.horizontal_top(|ui| {
                    ui.add_space(side_space);
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(content_width, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| workflow_content(app, ui),
                    );
                });
            }
        });
}

fn workflow_content(app: &mut CofferApp, ui: &mut egui::Ui) {
    match app.workflow {
        Workflow::Protect => protect_page(app, ui),
        Workflow::Open => open_page(app, ui),
        Workflow::Security => security_page(ui),
        Workflow::Settings => settings_page(app, ui),
    }
}

fn context_panel(app: &CofferApp, ui: &mut egui::Ui) {
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new("COFFER GUIDE")
            .small()
            .strong()
            .color(theme::TEXT_MUTED),
    );
    ui.add_space(12.0);

    egui::Frame::new()
        .fill(theme::SURFACE)
        .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
        .corner_radius(egui::CornerRadius::same(16))
        .inner_margin(18.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let (title, message, accent) = context_copy(app);
            ui.colored_label(accent, "●");
            ui.add_space(8.0);
            ui.heading(
                egui::RichText::new(title)
                    .size(18.0)
                    .color(theme::TEXT_PRIMARY),
            );
            ui.add_space(8.0);
            ui.label(egui::RichText::new(message).color(theme::TEXT_SECONDARY));
        });

    ui.add_space(14.0);
    egui::Frame::new()
        .fill(theme::SURFACE_RAISED.gamma_multiply(0.55))
        .corner_radius(egui::CornerRadius::same(14))
        .inner_margin(16.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                egui::RichText::new("Always local")
                    .strong()
                    .color(theme::TEXT_PRIMARY),
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("Files stay on this device. Coffer does not require an account or upload your content.")
                    .small()
                    .color(theme::TEXT_SECONDARY),
            );
        });
}

fn context_copy(app: &CofferApp) -> (&'static str, &'static str, egui::Color32) {
    match app.workflow {
        Workflow::Protect => match app.protect_stage {
            ProtectStage::SelectFile => (
                "Choose how the file is unlocked",
                "Create a new random key for this file, or deliberately reuse an existing Coffer key when several protected files must share access.",
                theme::ACCENT,
            ),
            ProtectStage::Review => (
                "Keep the key separate",
                "The key is required to restore the file. Store or send it through a different location or channel.",
                theme::WARNING,
            ),
            ProtectStage::Processing => (
                "Protecting locally",
                "Keep Coffer open while the operation completes. No content is uploaded.",
                theme::PRIMARY_HOVER,
            ),
            ProtectStage::Complete => (
                "Two outputs, one responsibility",
                "The protected file and key work together. Anyone with both can restore the original.",
                theme::SUCCESS,
            ),
        },
        Workflow::Open => match app.open_stage {
            OpenStage::SelectContainer => (
                "Start with the .coffer file",
                "This is the protected container. Its filename and exact original size will be encrypted in format v1.",
                theme::ACCENT,
            ),
            OpenStage::SelectKey => (
                "Now choose its key",
                "Coffer will authenticate the selected pair before any restored content is written.",
                theme::WARNING,
            ),
            OpenStage::Review => (
                "Authentication comes first",
                "A wrong key, damaged file, or modified container will fail before Coffer writes plaintext.",
                theme::ACCENT,
            ),
            OpenStage::Processing => (
                "Restoring locally",
                "The container is authenticated before the restored file is committed to disk.",
                theme::PRIMARY_HOVER,
            ),
            OpenStage::Complete => (
                "Restoration complete",
                "Open the restored file or reveal it in its destination when real file operations are connected.",
                theme::SUCCESS,
            ),
        },
        Workflow::Security => (
            "Security without mystery",
            "Coffer uses established cryptography and keeps sensitive filename metadata inside the encrypted payload.",
            theme::SUCCESS,
        ),
        Workflow::Settings => (
            "Secure defaults stay fixed",
            "Convenience can be adjusted, but cryptographic choices are intentionally not exposed as preferences.",
            theme::ACCENT,
        ),
    }
}

fn page_header(ui: &mut egui::Ui, eyebrow: &str, title: &str, description: &str) {
    ui.label(
        egui::RichText::new(eyebrow)
            .small()
            .strong()
            .color(theme::PRIMARY_HOVER),
    );
    ui.add_space(8.0);
    ui.heading(
        egui::RichText::new(title)
            .size(31.0)
            .strong()
            .color(theme::TEXT_PRIMARY),
    );
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(description)
            .size(15.0)
            .color(theme::TEXT_SECONDARY),
    );
}

fn protect_page(app: &mut CofferApp, ui: &mut egui::Ui) {
    page_header(
        ui,
        "PROTECT • ENCRYPT",
        "Protect a file",
        "Create an encrypted copy and a separate key. Your original file will not be changed.",
    );
    ui.add_space(18.0);
    widgets::workflow_steps(
        ui,
        protect_stage_index(app.protect_stage),
        &["Choose file", "Review", "Protect", "Done"],
    );
    ui.add_space(24.0);

    match app.protect_stage {
        ProtectStage::SelectFile => protect_select(app, ui),
        ProtectStage::Review => protect_review(app, ui),
        ProtectStage::Processing => processing_panel(ui, "Protecting your file…"),
        ProtectStage::Complete => protect_result(app, ui),
    }
}

fn protect_select(app: &mut CofferApp, ui: &mut egui::Ui) {
    match &app.source_file {
        Some(file) => {
            section_label(ui, "File to protect", "Ready for review");
            if widgets::file_card(ui, file) {
                app.clear_source_file();
            }
            ui.add_space(16.0);
            key_source_selector(app, ui);
        }
        None => {
            let response = widgets::drop_zone(
                ui,
                "Drop a file here",
                "Choose any local file. The original will remain untouched.",
                "Browse files",
            );
            if response.clicked() {
                app.select_source_file();
            }
        }
    }
}

fn key_source_selector(app: &mut CofferApp, ui: &mut egui::Ui) {
    section_label(ui, "Unlock method", "Choose a key for this protected file");
    let available = ui.available_width();

    if available >= 680.0 {
        ui.columns(2, |columns| {
            key_source_option(
                &mut columns[0],
                app.protect_key_source == ProtectKeySource::GenerateNew,
                "New random key",
                "Recommended",
                "Coffer creates a unique key for this file.",
                theme::PRIMARY,
                || app.set_protect_key_source(ProtectKeySource::GenerateNew),
            );
            key_source_option(
                &mut columns[1],
                app.protect_key_source == ProtectKeySource::Existing,
                "Existing key",
                "Advanced",
                "Protect this file using a key you already have.",
                theme::ACCENT,
                || app.set_protect_key_source(ProtectKeySource::Existing),
            );
        });
    } else {
        key_source_option(
            ui,
            app.protect_key_source == ProtectKeySource::GenerateNew,
            "New random key",
            "Recommended",
            "Coffer creates a unique key for this file.",
            theme::PRIMARY,
            || app.set_protect_key_source(ProtectKeySource::GenerateNew),
        );
        ui.add_space(10.0);
        key_source_option(
            ui,
            app.protect_key_source == ProtectKeySource::Existing,
            "Existing key",
            "Advanced",
            "Protect this file using a key you already have.",
            theme::ACCENT,
            || app.set_protect_key_source(ProtectKeySource::Existing),
        );
    }

    if app.protect_key_source == ProtectKeySource::Existing {
        ui.add_space(14.0);
        match app.protect_key_file.as_ref() {
            Some(file) => {
                if widgets::file_card(ui, file) {
                    app.clear_protect_key();
                }
            }
            None => {
                let response = widgets::drop_zone(
                    ui,
                    "Drop an existing .key file",
                    "Only a valid Coffer key will be accepted by the production workflow.",
                    "Browse Coffer keys",
                );
                if response.clicked() {
                    app.select_protect_key();
                }
                if app.scroll_to_protect_key {
                    ui.scroll_to_rect(response.rect, Some(egui::Align::Center));
                    app.scroll_to_protect_key = false;
                }
            }
        }
        ui.add_space(12.0);
        info_panel(
            ui,
            "Key reuse changes your risk",
            "Every file protected with this key can be opened by anyone who has it. A new random key provides stronger separation between files.",
            theme::WARNING,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn key_source_option(
    ui: &mut egui::Ui,
    selected: bool,
    title: &str,
    badge: &str,
    description: &str,
    accent: egui::Color32,
    on_select: impl FnOnce(),
) {
    let fill = if selected {
        accent.gamma_multiply(0.12)
    } else {
        theme::SURFACE
    };
    let stroke = if selected {
        egui::Stroke::new(1.5_f32, accent)
    } else {
        egui::Stroke::new(1.0_f32, theme::BORDER)
    };
    let response = egui::Frame::new()
        .fill(fill)
        .stroke(stroke)
        .corner_radius(egui::CornerRadius::same(15))
        .inner_margin(16.0)
        .show(ui, |ui| {
            ui.set_min_height(88.0);
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.colored_label(accent, if selected { "●" } else { "○" });
                ui.label(
                    egui::RichText::new(title)
                        .strong()
                        .color(theme::TEXT_PRIMARY),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(badge).small().strong().color(accent));
                });
            });
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(description)
                    .small()
                    .color(theme::TEXT_SECONDARY),
            );
        })
        .response
        .interact(egui::Sense::click());

    if response.clicked() && !selected {
        on_select();
    }
}

fn protect_review(app: &CofferApp, ui: &mut egui::Ui) {
    section_label(ui, "Review", "Confirm these details before continuing");
    if let Some(file) = app.source_file.as_ref() {
        widgets::file_card_readonly(ui, file);
        ui.add_space(16.0);
        let protected_name = format!("{}.coffer", file.name);
        let key_description = match app.protect_key_source {
            ProtectKeySource::GenerateNew => format!("{}.coffer.key", file.name),
            ProtectKeySource::Existing => app
                .protect_key_file
                .as_ref()
                .map(|key| format!("Reuse {}", key.name))
                .unwrap_or_else(|| "Existing key required".to_string()),
        };
        detail_panel(
            ui,
            "Protection plan",
            &[
                ("Protected copy", &protected_name),
                ("Unlock key", &key_description),
                ("Save location", "Next to the original (prototype)"),
                ("Original file", "Will not be changed"),
            ],
        );
        ui.add_space(16.0);
        info_panel(
            ui,
            "Keep the key safe",
            "A lost key cannot be recovered. Store or send the key separately from the protected file.",
            theme::WARNING,
        );
    }
}

fn open_page(app: &mut CofferApp, ui: &mut egui::Ui) {
    page_header(
        ui,
        "OPEN • DECRYPT",
        "Open a protected file",
        "Select the protected file and the separate key that was created with it.",
    );
    ui.add_space(18.0);
    widgets::workflow_steps(
        ui,
        open_stage_index(app.open_stage),
        &["Protected file", "Key", "Review", "Restore", "Done"],
    );
    ui.add_space(24.0);

    match app.open_stage {
        OpenStage::SelectContainer => open_container_select(app, ui),
        OpenStage::SelectKey => open_key_select(app, ui),
        OpenStage::Review => open_review(app, ui),
        OpenStage::Processing => processing_panel(ui, "Authenticating and restoring…"),
        OpenStage::Complete => open_result(app, ui),
    }
}

fn open_container_select(app: &mut CofferApp, ui: &mut egui::Ui) {
    section_label(ui, "Step 1 of 2", "Choose the protected file");
    let response = widgets::drop_zone(
        ui,
        "Drop a .coffer file here",
        "This is the protected file you want to restore.",
        "Browse protected files",
    );
    if response.clicked() {
        app.select_encrypted_file();
    }
    if app.key_file.is_some() {
        ui.add_space(14.0);
        info_panel(
            ui,
            "Key detected",
            "Your dropped key is preserved and will be used after you choose the protected file.",
            theme::SUCCESS,
        );
    }
}

fn open_key_select(app: &mut CofferApp, ui: &mut egui::Ui) {
    section_label(ui, "Step 2 of 2", "Choose the matching key");
    if let Some(container) = app.encrypted_file.as_ref() {
        widgets::compact_file_row(ui, "Protected file", container, || {});
        ui.add_space(14.0);
    }

    match &app.key_file {
        Some(file) => {
            if widgets::file_card(ui, file) {
                app.clear_key_file();
            }
            ui.add_space(14.0);
            info_panel(
                ui,
                "Ready to review",
                "Coffer will verify whether this key matches during restoration.",
                theme::SUCCESS,
            );
        }
        None => {
            let response = widgets::drop_zone(
                ui,
                "Drop the key here",
                "Choose the separate .key file created with this protected file.",
                "Browse key files",
            );
            if response.clicked() {
                app.select_key();
            }
        }
    }
}

fn open_review(app: &CofferApp, ui: &mut egui::Ui) {
    section_label(
        ui,
        "Review",
        "Coffer will authenticate the file before restoring anything",
    );
    if let Some(file) = app.encrypted_file.as_ref() {
        widgets::file_card_readonly(ui, file);
    }
    ui.add_space(12.0);
    if let Some(file) = app.key_file.as_ref() {
        widgets::file_card_readonly(ui, file);
    }
    ui.add_space(16.0);
    detail_panel(
        ui,
        "Restoration",
        &[
            ("Destination", "Ask before saving"),
            ("Authentication", "Verified before writing"),
            ("Existing files", "Never replaced silently"),
        ],
    );
}

fn processing_panel(ui: &mut egui::Ui, title: &str) {
    theme::card_frame().show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.vertical_centered(|ui| {
            ui.add_space(24.0);
            ui.spinner();
            ui.add_space(16.0);
            ui.heading(egui::RichText::new(title).color(theme::TEXT_PRIMARY));
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("Keep Coffer open until this operation finishes.")
                    .color(theme::TEXT_SECONDARY),
            );
            ui.add_space(24.0);
        });
    });
}

fn protect_result(app: &mut CofferApp, ui: &mut egui::Ui) {
    prototype_banner(ui);
    ui.add_space(16.0);
    result_heading(
        ui,
        "Protection preview complete",
        "These are the paths the production workflow will create.",
    );
    ui.add_space(16.0);
    let protected = display_path(app.encryption_output.as_ref());
    if app.protect_key_source == ProtectKeySource::GenerateNew {
        let key = display_path(app.key_output.as_ref());
        detail_panel(
            ui,
            "Planned output",
            &[("Protected file", &protected), ("New key file", &key)],
        );
    } else {
        let key = app
            .protect_key_file
            .as_ref()
            .map(|file| file.name.as_str())
            .unwrap_or("Existing key");
        detail_panel(
            ui,
            "Planned output",
            &[("Protected file", &protected), ("Key reused", key)],
        );
    }
    ui.add_space(16.0);
    info_panel(
        ui,
        "Important",
        "Keep the protected file and its key separate. Anyone with both can restore the original.",
        theme::WARNING,
    );
    ui.add_space(18.0);
    if ui.button("Protect another file").clicked() {
        app.reset_protect();
    }
}

fn open_result(app: &mut CofferApp, ui: &mut egui::Ui) {
    prototype_banner(ui);
    ui.add_space(16.0);
    result_heading(
        ui,
        "Restoration preview complete",
        "No file was written because cryptography is not connected yet.",
    );
    ui.add_space(16.0);
    detail_panel(
        ui,
        "Planned output",
        &[(
            "Restored file",
            &display_path(app.decryption_output.as_ref()),
        )],
    );
    ui.add_space(18.0);
    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                app.decrypted_text.is_some(),
                egui::Button::new("Preview sample text"),
            )
            .clicked()
        {
            app.open_secure_viewer();
        }
        if ui.button("Open another file").clicked() {
            app.reset_open();
        }
    });
}

fn action_bar(app: &mut CofferApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let message = action_message(app);
        if ui.available_width() >= 560.0 {
            ui.add(
                egui::Label::new(egui::RichText::new(message).color(theme::TEXT_SECONDARY))
                    .truncate(),
            );
        }
        ui.with_layout(
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| match app.workflow {
                Workflow::Protect => match app.protect_stage {
                    ProtectStage::SelectFile => {
                        if widgets::primary_button(ui, "Continue to review", app.can_protect())
                            .clicked()
                        {
                            app.review_protect();
                        }
                    }
                    ProtectStage::Review => {
                        if widgets::primary_button(ui, "Protect file", true).clicked() {
                            app.run_protect();
                        }
                        if ui.button("Back").clicked() {
                            app.back();
                        }
                    }
                    ProtectStage::Processing | ProtectStage::Complete => {}
                },
                Workflow::Open => match app.open_stage {
                    OpenStage::SelectContainer => {}
                    OpenStage::SelectKey => {
                        if widgets::primary_button(ui, "Continue to review", app.can_open())
                            .clicked()
                        {
                            app.review_open();
                        }
                        if ui.button("Change protected file").clicked() {
                            app.clear_encrypted_file();
                        }
                    }
                    OpenStage::Review => {
                        if widgets::primary_button(ui, "Restore file", true).clicked() {
                            app.run_open();
                        }
                        if ui.button("Back").clicked() {
                            app.back();
                        }
                    }
                    OpenStage::Processing | OpenStage::Complete => {}
                },
                Workflow::Security | Workflow::Settings => {}
            },
        );
    });
}

fn action_message(app: &CofferApp) -> &'static str {
    match app.workflow {
        Workflow::Protect => match app.protect_stage {
            ProtectStage::SelectFile if app.can_protect() => "Your file is ready for review",
            ProtectStage::SelectFile
                if app.source_file.is_some()
                    && app.protect_key_source == ProtectKeySource::Existing =>
            {
                "Choose the existing key to continue"
            }
            ProtectStage::SelectFile => "Choose one file to continue",
            ProtectStage::Review => "Review the output and key warning",
            ProtectStage::Processing => "Protecting locally",
            ProtectStage::Complete => "Prototype only — nothing was written",
        },
        Workflow::Open => match app.open_stage {
            OpenStage::SelectContainer => "Start with the protected .coffer file",
            OpenStage::SelectKey if app.can_open() => "Both files are ready for review",
            OpenStage::SelectKey => "Now choose its separate key",
            OpenStage::Review => "Authentication happens before restoration",
            OpenStage::Processing => "Restoring locally",
            OpenStage::Complete => "Prototype only — nothing was restored",
        },
        Workflow::Security | Workflow::Settings => "Coffer processes files locally",
    }
}

fn security_page(ui: &mut egui::Ui) {
    page_header(
        ui,
        "SECURITY",
        "Protection without mystery",
        "What Coffer protects, what you must keep safe, and what the technology does.",
    );
    ui.add_space(24.0);
    info_panel(
        ui,
        "Everything stays on this device",
        "Coffer is designed to process files locally. It does not require an account or upload your files.",
        theme::SUCCESS,
    );
    ui.add_space(16.0);
    detail_panel(
        ui,
        "How protection works",
        &[
            ("Encryption", "AES-256-GCM"),
            ("Key", "New random 256-bit key for every file"),
            ("Integrity", "Changes are detected before restoration"),
            ("Container", "Versioned .coffer format"),
        ],
    );
    ui.add_space(16.0);
    info_panel(
        ui,
        "Your key is essential",
        "Coffer has no recovery backdoor. If the key is lost, the protected file cannot be restored. Never send the protected file and key through the same channel.",
        theme::WARNING,
    );
    ui.add_space(16.0);
    theme::card_frame().show(ui, |ui| {
        ui.heading(egui::RichText::new("Plain-language promises").color(theme::TEXT_PRIMARY));
        ui.add_space(12.0);
        for text in [
            "Your original file is not changed during protection.",
            "A file is authenticated before restored content is written.",
            "Coffer does not claim it can recover a missing key.",
            "Temporary plaintext may exist in memory while a file is open.",
        ] {
            ui.horizontal(|ui| {
                ui.colored_label(theme::ACCENT, "✓");
                ui.label(egui::RichText::new(text).color(theme::TEXT_SECONDARY));
            });
        }
    });
}

fn settings_page(app: &mut CofferApp, ui: &mut egui::Ui) {
    page_header(
        ui,
        "SETTINGS",
        "Simple, secure defaults",
        "Control convenience and privacy without changing Coffer’s cryptography.",
    );
    ui.add_space(24.0);
    settings_group(ui, "Saving files", |ui| {
        setting_toggle(
            ui,
            &mut app.ask_for_output_location,
            "Ask where to save every time",
            "Choose a destination before Coffer writes output.",
        );
        setting_toggle(
            ui,
            &mut app.confirm_before_replace,
            "Confirm before replacing files",
            "Coffer never silently overwrites an existing file.",
        );
    });
    ui.add_space(16.0);
    settings_group(ui, "Privacy", |ui| {
        setting_toggle(
            ui,
            &mut app.offer_text_preview,
            "Offer previews for readable text",
            "Preview is optional and only available for supported text files.",
        );
        setting_toggle(
            ui,
            &mut app.clear_recent_locations,
            "Clear recent locations when Coffer closes",
            "Do not retain recently used folders between sessions.",
        );
    });
    ui.add_space(16.0);
    info_panel(
        ui,
        "Cryptography uses fixed secure defaults",
        "Algorithms, key size, and nonce handling are intentionally not configurable.",
        theme::ACCENT,
    );
}

fn settings_group(ui: &mut egui::Ui, title: &str, add: impl FnOnce(&mut egui::Ui)) {
    theme::card_frame().show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.heading(egui::RichText::new(title).color(theme::TEXT_PRIMARY));
        ui.add_space(12.0);
        add(ui);
    });
}

fn setting_toggle(ui: &mut egui::Ui, value: &mut bool, title: &str, description: &str) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new(title)
                    .strong()
                    .color(theme::TEXT_PRIMARY),
            );
            ui.label(
                egui::RichText::new(description)
                    .small()
                    .color(theme::TEXT_SECONDARY),
            );
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.toggle_value(value, "");
        });
    });
    ui.add_space(12.0);
}

fn section_label(ui: &mut egui::Ui, eyebrow: &str, title: &str) {
    ui.label(
        egui::RichText::new(eyebrow)
            .small()
            .strong()
            .color(theme::PRIMARY_HOVER),
    );
    ui.heading(
        egui::RichText::new(title)
            .size(20.0)
            .color(theme::TEXT_PRIMARY),
    );
    ui.add_space(12.0);
}

fn info_panel(ui: &mut egui::Ui, title: &str, message: &str, color: egui::Color32) {
    egui::Frame::new()
        .fill(color.gamma_multiply(0.08))
        .stroke(egui::Stroke::new(1.0_f32, color.gamma_multiply(0.65)))
        .corner_radius(egui::CornerRadius::same(14))
        .inner_margin(18.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(egui::RichText::new(title).strong().color(color));
            ui.add_space(5.0);
            ui.label(egui::RichText::new(message).color(theme::TEXT_SECONDARY));
        });
}

fn detail_panel(ui: &mut egui::Ui, title: &str, rows: &[(&str, &str)]) {
    theme::card_frame().show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.heading(
            egui::RichText::new(title)
                .size(18.0)
                .color(theme::TEXT_PRIMARY),
        );
        ui.add_space(14.0);
        for (label, value) in rows {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(*label).color(theme::TEXT_SECONDARY));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(*value)
                                .strong()
                                .color(theme::TEXT_PRIMARY),
                        )
                        .truncate(),
                    );
                });
            });
            ui.add_space(9.0);
        }
    });
}

fn prototype_banner(ui: &mut egui::Ui) {
    info_panel(
        ui,
        "Prototype operation",
        "The interface flow is complete, but real encryption and file writing are not connected yet.",
        theme::WARNING,
    );
}

fn result_heading(ui: &mut egui::Ui, title: &str, description: &str) {
    ui.colored_label(theme::SUCCESS, "✓  COMPLETE");
    ui.add_space(10.0);
    ui.heading(
        egui::RichText::new(title)
            .size(26.0)
            .color(theme::TEXT_PRIMARY),
    );
    ui.add_space(6.0);
    ui.label(egui::RichText::new(description).color(theme::TEXT_SECONDARY));
}

fn display_path(path: Option<&std::path::PathBuf>) -> String {
    path.map(|path| path.display().to_string())
        .unwrap_or_else(|| "Output path unavailable".to_string())
}

fn protect_stage_index(stage: ProtectStage) -> usize {
    match stage {
        ProtectStage::SelectFile => 0,
        ProtectStage::Review => 1,
        ProtectStage::Processing => 2,
        ProtectStage::Complete => 3,
    }
}

fn open_stage_index(stage: OpenStage) -> usize {
    match stage {
        OpenStage::SelectContainer => 0,
        OpenStage::SelectKey => 1,
        OpenStage::Review => 2,
        OpenStage::Processing => 3,
        OpenStage::Complete => 4,
    }
}
