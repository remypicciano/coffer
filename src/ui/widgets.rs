use eframe::egui;

use crate::ui::theme;

pub fn coffer_card(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    content: impl FnOnce(&mut egui::Ui),
) {
    theme::card_frame().show(ui, |ui| {
        ui.set_width(ui.available_width());

        ui.label(
            egui::RichText::new(title)
                .size(17.0)
                .strong()
                .color(theme::TEXT_PRIMARY),
        );

        ui.add_space(4.0);

        ui.label(
            egui::RichText::new(description)
                .size(13.0)
                .color(theme::TEXT_SECONDARY),
        );

        ui.add_space(16.0);

        content(ui);
    });
}

pub fn primary_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let button = egui::Button::new(
        egui::RichText::new(text)
            .size(16.0)
            .strong()
            .color(egui::Color32::WHITE),
    )
    .fill(theme::PRIMARY)
    .stroke(egui::Stroke::NONE)
    .corner_radius(egui::CornerRadius::same(theme::BUTTON_RADIUS));

    ui.add_sized([ui.available_width(), 48.0], button)
}

pub fn secondary_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add_sized(
        [ui.available_width(), 42.0],
        egui::Button::new(
            egui::RichText::new(text)
                .strong()
                .color(theme::TEXT_PRIMARY),
        )
        .fill(theme::SURFACE_RAISED)
        .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
        .corner_radius(egui::CornerRadius::same(theme::BUTTON_RADIUS)),
    )
}

pub fn status_pill(ui: &mut egui::Ui, status: &str) {
    let lowercase = status.to_lowercase();

    let color = if lowercase.contains("success") {
        theme::SUCCESS
    } else if lowercase.contains("missing") || lowercase.contains("error") {
        theme::DANGER
    } else if lowercase.contains("decrypting") {
        theme::WARNING
    } else {
        theme::ACCENT
    };

    egui::Frame::new()
        .fill(theme::SURFACE_RAISED)
        .corner_radius(egui::CornerRadius::same(20))
        .inner_margin(egui::Margin::symmetric(12, 7))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(color, "●");

                ui.label(
                    egui::RichText::new(status)
                        .size(13.0)
                        .strong()
                        .color(theme::TEXT_PRIMARY),
                );
            });
        });
}

pub fn selected_file_row(ui: &mut egui::Ui, filename: Option<&str>, button_text: &str) -> bool {
    let mut clicked = false;

    theme::raised_frame().show(ui, |ui| {
        ui.set_width(ui.available_width());

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(filename.unwrap_or("No file selected"))
                        .strong()
                        .color(if filename.is_some() {
                            theme::TEXT_PRIMARY
                        } else {
                            theme::TEXT_MUTED
                        }),
                );

                ui.label(
                    egui::RichText::new(if filename.is_some() {
                        "Ready to use"
                    } else {
                        "Choose a local file"
                    })
                    .small()
                    .color(theme::TEXT_SECONDARY),
                );
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(
                        egui::Button::new(button_text)
                            .fill(theme::PRIMARY)
                            .corner_radius(egui::CornerRadius::same(10)),
                    )
                    .clicked()
                {
                    clicked = true;
                }
            });
        });
    });

    clicked
}

pub fn footer(ui: &mut egui::Ui) {
    ui.add_space(22.0);

    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new("Local encryption. Your data stays on this device.")
                .small()
                .color(theme::TEXT_MUTED),
        );

        ui.add_space(4.0);

        ui.label(
            egui::RichText::new("Designed & Developed by John Doe")
                .small()
                .color(theme::TEXT_SECONDARY),
        );
    });
}
