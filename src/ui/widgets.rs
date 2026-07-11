use eframe::egui;

use crate::app::{Notice, NoticeKind, SelectedFile};
use crate::ui::theme;

pub fn primary_button(ui: &mut egui::Ui, text: &str, enabled: bool) -> egui::Response {
    ui.add_enabled(
        enabled,
        egui::Button::new(
            egui::RichText::new(text)
                .size(15.0)
                .strong()
                .color(egui::Color32::WHITE),
        )
        .fill(theme::PRIMARY)
        .stroke(egui::Stroke::NONE)
        .corner_radius(egui::CornerRadius::same(11))
        .min_size(egui::Vec2::new(168.0, 44.0)),
    )
}

pub fn nav_button(ui: &mut egui::Ui, label: &str, selected: bool) -> egui::Response {
    let fill = if selected {
        theme::SURFACE_RAISED
    } else {
        egui::Color32::TRANSPARENT
    };

    let stroke = if selected {
        egui::Stroke::new(1.0_f32, theme::BORDER)
    } else {
        egui::Stroke::NONE
    };

    ui.add_sized(
        [ui.available_width(), 42.0],
        egui::Button::new(egui::RichText::new(label).strong().color(if selected {
            theme::TEXT_PRIMARY
        } else {
            theme::TEXT_SECONDARY
        }))
        .fill(fill)
        .stroke(stroke)
        .corner_radius(egui::CornerRadius::same(10)),
    )
}

pub fn workflow_steps(ui: &mut egui::Ui, active_step: usize, labels: &[&str]) {
    ui.horizontal_wrapped(|ui| {
        for (index, label) in labels.iter().enumerate() {
            let complete = index < active_step;

            let active = index == active_step;

            let color = if complete {
                theme::SUCCESS
            } else if active {
                theme::PRIMARY
            } else {
                theme::TEXT_MUTED
            };

            egui::Frame::new()
                .fill(if active {
                    theme::SURFACE_RAISED
                } else {
                    egui::Color32::TRANSPARENT
                })
                .corner_radius(egui::CornerRadius::same(15))
                .inner_margin(egui::Margin::symmetric(9, 5))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.colored_label(color, if complete { "✓" } else { "●" });

                        ui.label(egui::RichText::new(*label).small().strong().color(
                            if active || complete {
                                theme::TEXT_PRIMARY
                            } else {
                                theme::TEXT_MUTED
                            },
                        ));
                    });
                });

            if index + 1 < labels.len() {
                ui.label(egui::RichText::new("—").color(theme::BORDER));
            }
        }
    });
}

pub fn drop_zone(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    browse_label: &str,
) -> egui::Response {
    let desired_size = egui::Vec2::new(ui.available_width(), 176.0);

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    let hovered = response.hovered() || !ui.input(|input| input.raw.hovered_files.is_empty());

    let fill = if hovered {
        theme::SURFACE_RAISED
    } else {
        theme::SURFACE
    };

    let border = if hovered {
        theme::PRIMARY
    } else {
        theme::BORDER
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(16),
        fill,
        egui::Stroke::new(if hovered { 2.0_f32 } else { 1.0_f32 }, border),
        egui::StrokeKind::Inside,
    );

    let mut child =
        ui.new_child(egui::UiBuilder::new().max_rect(rect.shrink(20.0)).layout(
            egui::Layout::top_down(egui::Align::Center).with_main_align(egui::Align::Center),
        ));

    child.heading(
        egui::RichText::new(title)
            .size(19.0)
            .strong()
            .color(theme::TEXT_PRIMARY),
    );

    child.add_space(8.0);

    child.label(egui::RichText::new(description).color(theme::TEXT_SECONDARY));

    child.add_space(16.0);

    child.label(
        egui::RichText::new(browse_label)
            .small()
            .strong()
            .color(theme::PRIMARY_HOVER),
    );

    response
}

pub fn file_card(ui: &mut egui::Ui, file: &SelectedFile) -> bool {
    let mut remove_clicked = false;

    egui::Frame::new()
        .fill(theme::SURFACE)
        .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
        .corner_radius(egui::CornerRadius::same(16))
        .inner_margin(18.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(&file.name)
                            .size(16.0)
                            .strong()
                            .color(theme::TEXT_PRIMARY),
                    );

                    ui.add_space(5.0);

                    ui.label(
                        egui::RichText::new(format!(
                            "{} • {}",
                            file.extension.to_uppercase(),
                            file.readable_size(),
                        ))
                        .small()
                        .color(theme::TEXT_SECONDARY),
                    );

                    ui.add_space(3.0);

                    ui.label(
                        egui::RichText::new(file.path.display().to_string())
                            .small()
                            .color(theme::TEXT_MUTED),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    let remove_button = egui::Button::new(
                        egui::RichText::new("×")
                            .size(18.0)
                            .strong()
                            .color(theme::TEXT_SECONDARY),
                    )
                    .fill(egui::Color32::TRANSPARENT)
                    .stroke(egui::Stroke::NONE)
                    .corner_radius(egui::CornerRadius::same(8));

                    if ui
                        .add_sized([30.0, 30.0], remove_button)
                        .on_hover_text("Remove selected file")
                        .clicked()
                    {
                        remove_clicked = true;
                    }
                });
            });
        });

    remove_clicked
}

pub fn file_card_readonly(ui: &mut egui::Ui, file: &SelectedFile) {
    file_details_frame(ui, file);
}

pub fn compact_file_row(
    ui: &mut egui::Ui,
    label: &str,
    file: &SelectedFile,
    _on_remove: impl FnOnce(),
) {
    egui::Frame::new()
        .fill(theme::SURFACE)
        .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
        .corner_radius(egui::CornerRadius::same(12))
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(label).small().color(theme::TEXT_MUTED));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(&file.name)
                                .strong()
                                .color(theme::TEXT_PRIMARY),
                        )
                        .truncate(),
                    );
                });
            });
        });
}

fn file_details_frame(ui: &mut egui::Ui, file: &SelectedFile) {
    egui::Frame::new()
        .fill(theme::SURFACE)
        .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
        .corner_radius(egui::CornerRadius::same(16))
        .inner_margin(18.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(&file.name)
                                .size(16.0)
                                .strong()
                                .color(theme::TEXT_PRIMARY),
                        )
                        .truncate(),
                    );
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new(format!(
                            "{} • {}",
                            file.extension.to_uppercase(),
                            file.readable_size()
                        ))
                        .small()
                        .color(theme::TEXT_SECONDARY),
                    );
                    ui.add_space(3.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(file.path.display().to_string())
                                .small()
                                .color(theme::TEXT_MUTED),
                        )
                        .truncate(),
                    )
                    .on_hover_text(file.path.display().to_string());
                });
            });
        });
}

pub fn empty_state(ui: &mut egui::Ui, title: &str, description: &str) {
    ui.vertical_centered(|ui| {
        ui.add_space(30.0);

        ui.heading(
            egui::RichText::new(title)
                .size(21.0)
                .color(theme::TEXT_PRIMARY),
        );

        ui.add_space(8.0);

        ui.label(egui::RichText::new(description).color(theme::TEXT_SECONDARY));

        ui.add_space(30.0);
    });
}

pub fn status_pill(ui: &mut egui::Ui, notice: &Notice) {
    let color = match notice.kind {
        NoticeKind::Info => theme::ACCENT,
        NoticeKind::Success => theme::SUCCESS,
        NoticeKind::Warning => theme::WARNING,
        NoticeKind::Error => theme::DANGER,
    };

    egui::Frame::new()
        .fill(theme::SURFACE_RAISED)
        .corner_radius(egui::CornerRadius::same(18))
        .inner_margin(egui::Margin::symmetric(11, 7))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(color, "●");

                ui.add(
                    egui::Label::new(
                        egui::RichText::new(&notice.message)
                            .small()
                            .strong()
                            .color(theme::TEXT_PRIMARY),
                    )
                    .wrap_mode(egui::TextWrapMode::Truncate),
                );
            });
        });
}
