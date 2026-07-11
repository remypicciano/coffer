use eframe::egui;

use crate::app::{CofferApp, OperationStage, Workflow};

use crate::ui::{theme, widgets};

pub fn show(app: &mut CofferApp, ctx: &egui::Context) {
    egui::SidePanel::left("coffer_sidebar")
        .exact_width(240.0)
        .resizable(false)
        .frame(
            egui::Frame::new()
                .fill(theme::SURFACE)
                .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
                .inner_margin(16.0),
        )
        .show(ctx, |ui| {
            sidebar(app, ui);
        });

    egui::TopBottomPanel::bottom("coffer_action_bar")
        .resizable(false)
        .frame(
            egui::Frame::new()
                .fill(theme::SURFACE)
                .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
                .inner_margin(egui::Margin::symmetric(24, 14)),
        )
        .show(ctx, |ui| {
            action_bar(app, ui);
        });

    egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(theme::BACKGROUND)
                .inner_margin(26.0),
        )
        .show(ctx, |ui| {
            content(app, ui);
        });
}

fn sidebar(app: &mut CofferApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add(
            egui::Image::new(egui::include_image!("../../assets/Pastelito_img.webp"))
                .fit_to_exact_size(egui::Vec2::splat(64.0))
                .corner_radius(14.0),
        );

        ui.add_space(10.0);

        ui.heading(
            egui::RichText::new("Coffer")
                .size(25.0)
                .strong()
                .color(theme::TEXT_PRIMARY),
        );

        ui.label(
            egui::RichText::new("Private file protection")
                .small()
                .color(theme::TEXT_SECONDARY),
        );
    });

    ui.add_space(28.0);

    nav_item(app, ui, Workflow::Encrypt, "Encrypt");

    nav_item(app, ui, Workflow::Decrypt, "Decrypt");

    ui.add_space(14.0);
    ui.separator();
    ui.add_space(14.0);

    nav_item(app, ui, Workflow::Security, "Security");

    nav_item(app, ui, Workflow::Settings, "Settings");

    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        ui.label(
            egui::RichText::new("Local • Offline • Private")
                .small()
                .color(theme::TEXT_MUTED),
        );

        ui.add_space(10.0);

        ui.horizontal_centered(|ui| {
            widgets::status_pill(ui, &app.status);
        });
    });
}

fn nav_item(app: &mut CofferApp, ui: &mut egui::Ui, workflow: Workflow, label: &str) {
    if widgets::nav_button(ui, label, app.workflow == workflow).clicked() {
        app.navigate(workflow);
    }
}

fn content(app: &mut CofferApp, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let width = ui.available_width().min(900.0);

            ui.set_max_width(width);

            match app.workflow {
                Workflow::Encrypt => {
                    encrypt_page(app, ui);
                }

                Workflow::Decrypt => {
                    decrypt_page(app, ui);
                }

                Workflow::Security => {
                    security_placeholder(ui);
                }

                Workflow::Settings => {
                    settings_placeholder(ui);
                }
            }
        });
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

fn encrypt_page(app: &mut CofferApp, ui: &mut egui::Ui) {
    page_header(
        ui,
        "ENCRYPT",
        "Protect a local file",
        "Choose a file, review the operation, and create a protected Coffer container.",
    );

    ui.add_space(18.0);

    widgets::workflow_steps(
        ui,
        stage_index(app.stage),
        &["Select", "Review", "Process", "Complete"],
    );

    ui.add_space(24.0);

    if app.stage == OperationStage::Complete {
        encrypt_result(app, ui);
        return;
    }

    match &app.source_file {
        Some(file) => {
            if widgets::file_card(ui, file) {
                app.clear_source_file();
            }
        }

        None => {
            let response = widgets::drop_zone(
                ui,
                "Drop a file to protect",
                "Any local file type can be selected.",
                "Click to browse",
            );

            if response.clicked() {
                app.select_source_file();
            }
        }
    }

    ui.add_space(20.0);

    review_panel(
        ui,
        "Encryption settings",
        &[
            ("Container", ".coffer"),
            ("Algorithm", "AES-256-GCM"),
            ("Key", "Separate random key"),
        ],
    );
}

fn decrypt_page(app: &mut CofferApp, ui: &mut egui::Ui) {
    page_header(
        ui,
        "DECRYPT",
        "Open protected content",
        "Choose a Coffer container and its matching key.",
    );

    ui.add_space(18.0);

    widgets::workflow_steps(
        ui,
        stage_index(app.stage),
        &["Select", "Review", "Process", "Complete"],
    );

    ui.add_space(24.0);

    if app.stage == OperationStage::Complete {
        decrypt_result(app, ui);
        return;
    }

    if ui.available_width() >= 620.0 {
        ui.columns(2, |columns| {
            file_picker_column(app, &mut columns[0], true);

            file_picker_column(app, &mut columns[1], false);
        });
    } else {
        file_picker_column(app, ui, true);

        ui.add_space(theme::SECTION_SPACING);

        file_picker_column(app, ui, false);
    }

    ui.add_space(20.0);

    review_panel(
        ui,
        "Decryption readiness",
        &[
            (
                "Container",
                if app.encrypted_file.is_some() {
                    "Selected"
                } else {
                    "Required"
                },
            ),
            (
                "Key",
                if app.key_file.is_some() {
                    "Selected"
                } else {
                    "Required"
                },
            ),
            ("Integrity", "Verified during decryption"),
        ],
    );
}

fn file_picker_column(app: &mut CofferApp, ui: &mut egui::Ui, encrypted: bool) {
    let file = if encrypted {
        app.encrypted_file.as_ref()
    } else {
        app.key_file.as_ref()
    };

    match file {
        Some(file) => {
            let remove_clicked = widgets::file_card(ui, file);

            if remove_clicked {
                if encrypted {
                    app.clear_encrypted_file();
                } else {
                    app.clear_key_file();
                }
            }
        }

        None => {
            let (title, description) = if encrypted {
                ("Drop a .coffer file", "The encrypted container to open.")
            } else {
                (
                    "Drop its key file",
                    "The separate key associated with the container.",
                )
            };

            let response = widgets::drop_zone(ui, title, description, "Click to browse");

            if response.clicked() {
                if encrypted {
                    app.select_encrypted_file();
                } else {
                    app.select_key();
                }
            }
        }
    }
}

fn action_bar(app: &mut CofferApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let message = match app.workflow {
            Workflow::Encrypt => {
                if app.can_encrypt() {
                    "Ready to encrypt"
                } else {
                    "Choose a file to continue"
                }
            }

            Workflow::Decrypt => {
                if app.can_decrypt() {
                    "Ready to decrypt"
                } else {
                    "Choose a container and key"
                }
            }

            Workflow::Security | Workflow::Settings => "Coffer processes files locally",
        };

        ui.label(egui::RichText::new(message).color(theme::TEXT_SECONDARY));

        ui.with_layout(
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| match app.workflow {
                Workflow::Encrypt => {
                    if app.stage != OperationStage::Complete
                        && widgets::primary_button(ui, "Encrypt file", app.can_encrypt()).clicked()
                    {
                        app.run_encrypt();
                    }
                }

                Workflow::Decrypt => {
                    if app.stage != OperationStage::Complete
                        && widgets::primary_button(ui, "Decrypt file", app.can_decrypt()).clicked()
                    {
                        app.run_decrypt();
                    }
                }

                Workflow::Security | Workflow::Settings => {}
            },
        );
    });
}

fn encrypt_result(app: &mut CofferApp, ui: &mut egui::Ui) {
    let output_path = app
        .encryption_output
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "Output path unavailable".to_string());

    result_panel(
        ui,
        "Encryption complete",
        "The protected container was created successfully.",
        &output_path,
    );
}

fn decrypt_result(app: &mut CofferApp, ui: &mut egui::Ui) {
    let output_path = app
        .decryption_output
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "Restored content".to_string());

    result_panel(
        ui,
        "Decryption complete",
        "The container passed authentication and the content is ready.",
        &output_path,
    );

    ui.add_space(16.0);

    let viewer_available = app.decrypted_text.is_some();

    if ui
        .add_enabled(
            viewer_available,
            egui::Button::new("Open Secure Viewer")
                .fill(theme::PRIMARY)
                .corner_radius(egui::CornerRadius::same(11))
                .min_size(egui::Vec2::new(220.0, 44.0)),
        )
        .clicked()
    {
        app.open_secure_viewer();
    }
}

fn result_panel(ui: &mut egui::Ui, title: &str, description: &str, path: &str) {
    egui::Frame::new()
        .fill(theme::SURFACE)
        .stroke(egui::Stroke::new(1.0_f32, theme::SUCCESS))
        .corner_radius(egui::CornerRadius::same(18))
        .inner_margin(24.0)
        .show(ui, |ui| {
            ui.colored_label(theme::SUCCESS, "● COMPLETE");

            ui.add_space(12.0);

            ui.heading(
                egui::RichText::new(title)
                    .size(25.0)
                    .color(theme::TEXT_PRIMARY),
            );

            ui.add_space(8.0);

            ui.label(egui::RichText::new(description).color(theme::TEXT_SECONDARY));

            ui.add_space(18.0);

            ui.label(
                egui::RichText::new(path)
                    .monospace()
                    .color(theme::TEXT_PRIMARY),
            );
        });
}

fn review_panel(ui: &mut egui::Ui, title: &str, rows: &[(&str, &str)]) {
    egui::Frame::new()
        .fill(theme::SURFACE)
        .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
        .corner_radius(egui::CornerRadius::same(16))
        .inner_margin(20.0)
        .show(ui, |ui| {
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
                        ui.label(
                            egui::RichText::new(*value)
                                .strong()
                                .color(theme::TEXT_PRIMARY),
                        );
                    });
                });

                ui.add_space(8.0);
            }
        });
}

fn security_placeholder(ui: &mut egui::Ui) {
    page_header(
        ui,
        "SECURITY",
        "Security details",
        "A dedicated explanation of Coffer’s cryptographic model will live here.",
    );

    ui.add_space(24.0);

    widgets::empty_state(
        ui,
        "Security panel planned",
        "This page will document AES-256-GCM, key separation, integrity verification, warnings, and the .coffer container format.",
    );
}

fn settings_placeholder(ui: &mut egui::Ui) {
    page_header(
        ui,
        "SETTINGS",
        "Application preferences",
        "Control output locations, appearance, overwrite behavior, and session wiping.",
    );

    ui.add_space(24.0);

    widgets::empty_state(
        ui,
        "Settings are coming next",
        "Coffer currently uses secure defaults without requiring configuration.",
    );
}

fn stage_index(stage: OperationStage) -> usize {
    match stage {
        OperationStage::SelectFiles => 0,
        OperationStage::Review => 1,
        OperationStage::Processing => 2,
        OperationStage::Complete => 3,
    }
}
