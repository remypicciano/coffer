use eframe::egui;

use crate::app::SelectedFile;
use crate::ui::theme;

pub fn primary_button(ui: &mut egui::Ui, text: &str, enabled: bool) -> egui::Response {
    let response = ui.add_enabled(
        enabled,
        egui::Button::new(
            egui::RichText::new(text)
                .size(15.0)
                .strong()
                .color(theme::on_primary()),
        )
        .fill(theme::primary())
        .stroke(egui::Stroke::NONE)
        .corner_radius(egui::CornerRadius::same(8))
        .min_size(egui::Vec2::new(168.0, 44.0)),
    );
    if response.hovered() && enabled {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    response
}

pub fn workflow_steps(ui: &mut egui::Ui, active_step: usize, labels: &[&str]) {
    let compact = ui.available_width() < 620.0;
    ui.horizontal_wrapped(|ui| {
        for (index, label) in labels.iter().enumerate() {
            let complete = index < active_step;

            let active = index == active_step;

            let color = if complete {
                theme::success()
            } else if active {
                theme::primary()
            } else {
                theme::text_muted()
            };

            egui::Frame::new()
                .fill(if active {
                    theme::surface_raised()
                } else {
                    egui::Color32::TRANSPARENT
                })
                .corner_radius(egui::CornerRadius::same(8))
                .inner_margin(egui::Margin::symmetric(9, 5))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.colored_label(
                            color,
                            if complete {
                                "OK".to_string()
                            } else {
                                format!("{:02}", index + 1)
                            },
                        );

                        let displayed_label = if compact && index != active_step {
                            (index + 1).to_string()
                        } else {
                            (*label).to_string()
                        };

                        ui.label(egui::RichText::new(displayed_label).small().strong().color(
                            if active || complete {
                                theme::text_primary()
                            } else {
                                theme::text_muted()
                            },
                        ));
                    });
                });

            if index + 1 < labels.len() {
                ui.label(egui::RichText::new("-").color(theme::border()));
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
    let desired_size = egui::Vec2::new(ui.available_width(), 190.0);

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    let hovered = response.hovered() || !ui.input(|input| input.raw.hovered_files.is_empty());
    let hover_amount = ui.ctx().animate_bool(response.id, hovered);

    let fill = theme::surface().lerp_to_gamma(theme::primary().gamma_multiply(0.10), hover_amount);

    let border = if hovered {
        theme::primary()
    } else {
        theme::border()
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(8),
        fill,
        egui::Stroke::new(1.0 + hover_amount, border),
        egui::StrokeKind::Inside,
    );

    if hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        ui.ctx().request_repaint();
    }

    let center = rect.center();
    ui.painter().text(
        center - egui::vec2(0.0, 15.0),
        egui::Align2::CENTER_CENTER,
        format!("{title} or {browse_label}"),
        egui::FontId::proportional(20.0),
        if hovered {
            theme::primary_hover()
        } else {
            theme::text_primary()
        },
    );
    ui.painter().text(
        center + egui::vec2(0.0, 18.0),
        egui::Align2::CENTER_CENTER,
        description,
        egui::FontId::proportional(14.0),
        theme::text_muted(),
    );

    response
}

pub fn file_card(ui: &mut egui::Ui, file: &SelectedFile) -> bool {
    let mut remove_clicked = false;

    egui::Frame::new()
        .fill(theme::surface())
        .stroke(egui::Stroke::new(1.0_f32, theme::border()))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(18.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(&file.name)
                            .size(16.0)
                            .strong()
                            .color(theme::text_primary()),
                    );

                    ui.add_space(5.0);

                    ui.label(
                        egui::RichText::new(format!(
                            "{} | {}",
                            file.extension.to_uppercase(),
                            file.readable_size(),
                        ))
                        .small()
                        .color(theme::text_secondary()),
                    );

                    ui.add_space(3.0);

                    ui.label(
                        egui::RichText::new(file.path.display().to_string())
                            .small()
                            .color(theme::text_muted()),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    let remove_button = egui::Button::new(
                        egui::RichText::new("Remove")
                            .small()
                            .strong()
                            .color(theme::text_secondary()),
                    )
                    .fill(egui::Color32::TRANSPARENT)
                    .stroke(egui::Stroke::NONE)
                    .corner_radius(egui::CornerRadius::same(8));

                    if ui
                        .add_sized([72.0, 30.0], remove_button)
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

pub fn compact_file_row(
    ui: &mut egui::Ui,
    label: &str,
    file: &SelectedFile,
    _on_remove: impl FnOnce(),
) {
    egui::Frame::new()
        .fill(theme::surface())
        .stroke(egui::Stroke::new(1.0_f32, theme::border()))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(label)
                        .small()
                        .color(theme::text_muted()),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(&file.name)
                                .strong()
                                .color(theme::text_primary()),
                        )
                        .truncate(),
                    );
                });
            });
        });
}
