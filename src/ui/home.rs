use eframe::egui;

use crate::app::CofferApp;
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

                            header(ui);

                            ui.add_space(30.0);

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

                            ui.add_space(24.0);
                        });
                    });
                });
        });
}

fn header(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        egui::Frame::new()
            .fill(theme::PRIMARY)
            .corner_radius(egui::CornerRadius::same(18))
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("C")
                        .size(30.0)
                        .strong()
                        .color(egui::Color32::WHITE),
                );
            });

        ui.add_space(16.0);

        ui.heading(
            egui::RichText::new("Coffer")
                .size(34.0)
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
        "Choose the key file associated with this encrypted file.",
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

        ui.horizontal(|ui| {
            widgets::status_pill(ui, &app.status);
        });

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
