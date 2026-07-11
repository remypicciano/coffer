use eframe::egui;

use crate::app::CofferApp;
use crate::ui::theme;

pub fn show(app: &mut CofferApp, ctx: &egui::Context) {
    show_success_dialog(app, ctx);
    show_error_dialog(app, ctx);
    show_secure_viewer(app, ctx);
}

fn dialog_frame(ctx: &egui::Context) -> egui::Frame {
    egui::Frame::window(&ctx.style())
        .fill(theme::SURFACE)
        .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
        .corner_radius(egui::CornerRadius::same(18))
        .inner_margin(24.0)
}

fn show_success_dialog(app: &mut CofferApp, ctx: &egui::Context) {
    if !app.show_success {
        return;
    }

    let mut open = true;
    let mut open_viewer = false;

    egui::Window::new("Decryption complete")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(400.0)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .frame(dialog_frame(ctx))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("✓")
                        .size(40.0)
                        .strong()
                        .color(theme::SUCCESS),
                );

                ui.add_space(10.0);

                ui.heading(egui::RichText::new("File decrypted").color(theme::TEXT_PRIMARY));

                ui.add_space(8.0);

                ui.label(
                    egui::RichText::new("Your decrypted content is ready to view.")
                        .color(theme::TEXT_SECONDARY),
                );

                ui.add_space(22.0);

                if ui
                    .add_sized(
                        [ui.available_width(), 46.0],
                        egui::Button::new(
                            egui::RichText::new("Open secure viewer")
                                .strong()
                                .color(egui::Color32::WHITE),
                        )
                        .fill(theme::PRIMARY)
                        .corner_radius(egui::CornerRadius::same(12)),
                    )
                    .clicked()
                {
                    open_viewer = true;
                }
            });
        });

    if open_viewer {
        app.show_success = false;
        app.show_viewer = true;
    } else if !open {
        app.show_success = false;
    }
}

fn show_error_dialog(app: &mut CofferApp, ctx: &egui::Context) {
    if !app.show_error {
        return;
    }

    let mut open = true;
    let mut dismiss = false;

    egui::Window::new("Unable to continue")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(400.0)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .frame(dialog_frame(ctx))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("!")
                        .size(40.0)
                        .strong()
                        .color(theme::DANGER),
                );

                ui.add_space(10.0);

                ui.heading(egui::RichText::new("Missing file or key").color(theme::TEXT_PRIMARY));

                ui.add_space(8.0);

                ui.label(
                    egui::RichText::new(
                        "Select both an encrypted file and a key before decrypting.",
                    )
                    .color(theme::TEXT_SECONDARY),
                );

                ui.add_space(22.0);

                if ui
                    .add_sized(
                        [ui.available_width(), 42.0],
                        egui::Button::new("Close").corner_radius(egui::CornerRadius::same(12)),
                    )
                    .clicked()
                {
                    dismiss = true;
                }
            });
        });

    if dismiss || !open {
        app.show_error = false;
    }
}

fn show_secure_viewer(app: &mut CofferApp, ctx: &egui::Context) {
    if !app.show_viewer {
        return;
    }

    let mut open = true;
    let mut wipe_requested = false;

    egui::Window::new("Secure viewer")
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .default_size([480.0, 500.0])
        .min_size([360.0, 340.0])
        .max_size([720.0, 680.0])
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .frame(dialog_frame(ctx))
        .show(ctx, |ui| {
            ui.set_min_width(320.0);

            ui.horizontal(|ui| {
                ui.colored_label(theme::WARNING, "●");

                ui.label(
                    egui::RichText::new("Temporary secure session")
                        .strong()
                        .color(theme::WARNING),
                );
            });

            ui.add_space(6.0);

            ui.label(
                egui::RichText::new(
                    "Closing this viewer removes the decrypted content from the application state.",
                )
                .small()
                .color(theme::TEXT_SECONDARY),
            );

            ui.add_space(16.0);

            let reserved_for_footer = 74.0;

            let viewer_height = (ui.available_height() - reserved_for_footer).max(160.0);

            egui::Frame::new()
                .fill(theme::BACKGROUND)
                .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
                .corner_radius(egui::CornerRadius::same(14))
                .inner_margin(18.0)
                .show(ui, |ui| {
                    ui.set_min_height(viewer_height);

                    egui::ScrollArea::vertical()
                        .max_height(viewer_height)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            if let Some(text) = app.decrypted_text.as_deref() {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(text)
                                            .monospace()
                                            .color(theme::TEXT_PRIMARY),
                                    )
                                    .wrap(),
                                );
                            } else {
                                ui.label(
                                    egui::RichText::new("No decrypted content is available.")
                                        .italics()
                                        .color(theme::TEXT_SECONDARY),
                                );
                            }
                        });
                });

            ui.add_space(16.0);

            if ui
                .add_sized(
                    [ui.available_width(), 44.0],
                    egui::Button::new(
                        egui::RichText::new("Close and wipe")
                            .strong()
                            .color(egui::Color32::WHITE),
                    )
                    .fill(theme::DANGER)
                    .corner_radius(egui::CornerRadius::same(12)),
                )
                .clicked()
            {
                wipe_requested = true;
            }
        });

    if wipe_requested || !open {
        app.decrypted_text = None;
        app.show_viewer = false;
        app.show_success = false;
        app.progress = 0.0;
        app.status = "Ready".into();
    }
}
