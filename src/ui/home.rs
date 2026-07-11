use eframe::egui;

use crate::app::{CofferApp, Workflow};

use crate::ui::{theme, widgets};

pub fn show(app: &mut CofferApp, ctx: &egui::Context) {
    egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(theme::BACKGROUND)
                .inner_margin(24.0),
        )
        .show(ctx, |ui| {
            let available_width = ui.available_width();

            let content_width = available_width.min(theme::MAX_CONTENT_WIDTH);

            let side_space = ((available_width - content_width) / 2.0).max(0.0);

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(side_space);

                        ui.vertical(|ui| {
                            ui.set_width(content_width);

                            match app.workflow {
                                Workflow::Home => {
                                    show_landing(app, ui);
                                }

                                Workflow::Encrypt => {
                                    show_encrypt(app, ui);
                                }

                                Workflow::Decrypt => {
                                    show_decrypt(app, ui);
                                }
                            }
                        });
                    });
                });
        });
}

fn brand_header(ui: &mut egui::Ui, compact: bool) {
    ui.vertical_centered(|ui| {
        let image_size = if compact { 72.0 } else { 104.0 };

        ui.add(
            egui::Image::new(egui::include_image!("../../assets/Pastelito_img.webp"))
                .fit_to_exact_size(egui::Vec2::splat(image_size))
                .corner_radius(18.0),
        );

        ui.add_space(if compact { 10.0 } else { 16.0 });

        ui.heading(
            egui::RichText::new("Coffer")
                .size(if compact { 28.0 } else { 38.0 })
                .strong()
                .color(theme::TEXT_PRIMARY),
        );

        ui.add_space(6.0);

        ui.label(
            egui::RichText::new("Private file protection, built for local use.")
                .size(15.0)
                .color(theme::TEXT_SECONDARY),
        );
    });
}

fn show_landing(app: &mut CofferApp, ui: &mut egui::Ui) {
    ui.add_space(18.0);

    brand_header(ui, false);

    ui.add_space(30.0);

    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new("What would you like to do?")
                .size(22.0)
                .strong()
                .color(theme::TEXT_PRIMARY),
        );

        ui.add_space(6.0);

        ui.label(
            egui::RichText::new("Choose a secure workflow to begin.").color(theme::TEXT_SECONDARY),
        );
    });

    ui.add_space(22.0);

    let use_columns = ui.available_width() >= 620.0;

    if use_columns {
        ui.columns(2, |columns| {
            workflow_card(
                &mut columns[0],
                "Protect a file",
                "Encrypt",
                "Turn a local file into an authenticated Coffer container.",
                theme::PRIMARY,
                || {
                    app.open_encrypt_workflow();
                },
            );

            workflow_card(
                &mut columns[1],
                "Open a file",
                "Decrypt",
                "Use the correct key to securely reveal protected content.",
                theme::ACCENT,
                || {
                    app.open_decrypt_workflow();
                },
            );
        });
    } else {
        workflow_card(
            ui,
            "Protect a file",
            "Encrypt",
            "Turn a local file into an authenticated Coffer container.",
            theme::PRIMARY,
            || {
                app.open_encrypt_workflow();
            },
        );

        ui.add_space(theme::SECTION_SPACING);

        workflow_card(
            ui,
            "Open a file",
            "Decrypt",
            "Use the correct key to securely reveal protected content.",
            theme::ACCENT,
            || {
                app.open_decrypt_workflow();
            },
        );
    }

    ui.add_space(22.0);

    security_strip(ui);

    widgets::footer(ui);

    ui.add_space(20.0);
}

fn workflow_card(
    ui: &mut egui::Ui,
    eyebrow: &str,
    title: &str,
    description: &str,
    accent: egui::Color32,
    action: impl FnOnce(),
) {
    let mut clicked = false;

    egui::Frame::new()
        .fill(theme::SURFACE)
        .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
        .corner_radius(egui::CornerRadius::same(20))
        .inner_margin(24.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            ui.set_min_height(230.0);

            ui.label(
                egui::RichText::new(eyebrow)
                    .size(12.0)
                    .strong()
                    .color(accent),
            );

            ui.add_space(14.0);

            ui.heading(
                egui::RichText::new(title)
                    .size(27.0)
                    .strong()
                    .color(theme::TEXT_PRIMARY),
            );

            ui.add_space(10.0);

            ui.label(
                egui::RichText::new(description)
                    .size(14.0)
                    .color(theme::TEXT_SECONDARY),
            );

            ui.add_space(26.0);

            if ui
                .add_sized(
                    [ui.available_width(), 46.0],
                    egui::Button::new(
                        egui::RichText::new(format!("Continue to {title}"))
                            .strong()
                            .color(egui::Color32::WHITE),
                    )
                    .fill(accent)
                    .corner_radius(egui::CornerRadius::same(12)),
                )
                .clicked()
            {
                clicked = true;
            }
        });

    if clicked {
        action();
    }
}

fn security_strip(ui: &mut egui::Ui) {
    theme::raised_frame().show(ui, |ui| {
        ui.set_width(ui.available_width());

        ui.columns(3, |columns| {
            security_item(&mut columns[0], "Local", "Files remain on this device.");

            security_item(
                &mut columns[1],
                "Authenticated",
                "Tampering causes decryption to fail.",
            );

            security_item(
                &mut columns[2],
                "Temporary",
                "Plaintext is held only for the session.",
            );
        });
    });
}

fn security_item(ui: &mut egui::Ui, title: &str, description: &str) {
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new(title)
                .strong()
                .color(theme::TEXT_PRIMARY),
        );

        ui.add_space(4.0);

        ui.label(
            egui::RichText::new(description)
                .small()
                .color(theme::TEXT_SECONDARY),
        );
    });
}

fn workflow_topbar(app: &mut CofferApp, ui: &mut egui::Ui, title: &str) {
    ui.horizontal(|ui| {
        if ui
            .add(
                egui::Button::new("Back")
                    .fill(theme::SURFACE_RAISED)
                    .corner_radius(egui::CornerRadius::same(10)),
            )
            .clicked()
        {
            app.return_home();
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(title)
                    .strong()
                    .color(theme::TEXT_SECONDARY),
            );
        });
    });
}

fn show_decrypt(app: &mut CofferApp, ui: &mut egui::Ui) {
    workflow_topbar(app, ui, "Decrypt workflow");

    ui.add_space(20.0);

    brand_header(ui, true);

    ui.add_space(26.0);

    file_section(app, ui);

    ui.add_space(theme::SECTION_SPACING);

    key_section(app, ui);

    ui.add_space(theme::SECTION_SPACING);

    status_section(app, ui);

    ui.add_space(24.0);

    if widgets::primary_button(ui, "Decrypt file").clicked() {
        app.decrypt();
    }

    widgets::footer(ui);

    ui.add_space(20.0);
}

fn show_encrypt(app: &mut CofferApp, ui: &mut egui::Ui) {
    workflow_topbar(app, ui, "Encrypt workflow");

    ui.add_space(20.0);

    brand_header(ui, true);

    ui.add_space(26.0);

    widgets::coffer_card(
        ui,
        "Encryption is coming next",
        "The interface is ready for the plaintext-file, key-generation, and output controls.",
        |ui| {
            ui.label(
                egui::RichText::new("Planned steps")
                    .strong()
                    .color(theme::TEXT_PRIMARY),
            );

            ui.add_space(10.0);

            ui.label(
                egui::RichText::new(
                    "1. Choose a file to protect\n\
                     2. Generate or select a key\n\
                     3. Choose an output location\n\
                     4. Encrypt and verify the result",
                )
                .color(theme::TEXT_SECONDARY),
            );
        },
    );

    ui.add_space(18.0);

    security_strip(ui);

    widgets::footer(ui);
}

fn file_section(app: &mut CofferApp, ui: &mut egui::Ui) {
    widgets::coffer_card(
        ui,
        "Encrypted file",
        "Select the Coffer file you want to open.",
        |ui| {
            let filename = app
                .encrypted_file
                .as_ref()
                .and_then(|path| path.file_name())
                .and_then(|name| name.to_str());

            if widgets::selected_file_row(ui, filename, "Choose file") {
                app.select_file();
            }
        },
    );
}

fn key_section(app: &mut CofferApp, ui: &mut egui::Ui) {
    widgets::coffer_card(
        ui,
        "Decryption key",
        "Choose the key associated with this encrypted file.",
        |ui| {
            let filename = app
                .key_file
                .as_ref()
                .and_then(|path| path.file_name())
                .and_then(|name| name.to_str());

            if widgets::selected_file_row(ui, filename, "Choose key") {
                app.select_key();
            }
        },
    );
}

fn status_section(app: &CofferApp, ui: &mut egui::Ui) {
    theme::card_frame().show(ui, |ui| {
        ui.set_width(ui.available_width());

        widgets::status_pill(ui, &app.status);

        if app.progress > 0.0 {
            ui.add_space(14.0);

            ui.add(
                egui::ProgressBar::new(app.progress)
                    .desired_width(ui.available_width())
                    .show_percentage(),
            );
        }
    });
}
