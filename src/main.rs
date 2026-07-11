mod app;
mod crypto;
mod error;
mod key;
mod paths;
mod ui;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 820.0])
            .with_min_inner_size([760.0, 600.0])
            .with_resizable(true),

        ..Default::default()
    };

    eframe::run_native(
        "Coffer",
        options,
        Box::new(|cc| {
            let ctx = &cc.egui_ctx;
            egui_extras::install_image_loaders(ctx);

            let mut visuals = egui::Visuals::dark();

            visuals.panel_fill = ui::theme::BACKGROUND;

            visuals.window_fill = ui::theme::SURFACE;

            visuals.extreme_bg_color = ui::theme::BACKGROUND;

            visuals.faint_bg_color = ui::theme::SURFACE_RAISED;

            visuals.selection.bg_fill = ui::theme::PRIMARY;

            visuals.selection.stroke.color = egui::Color32::WHITE;

            visuals.widgets.inactive.bg_fill = ui::theme::SURFACE_RAISED;

            visuals.widgets.inactive.weak_bg_fill = ui::theme::SURFACE_RAISED;

            visuals.widgets.hovered.bg_fill = ui::theme::PRIMARY_HOVER;

            visuals.widgets.active.bg_fill = ui::theme::PRIMARY;

            visuals.widgets.noninteractive.bg_fill = ui::theme::SURFACE;

            ctx.set_visuals(visuals);

            let mut style = (*ctx.style()).clone();

            style.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);

            style.spacing.button_padding = egui::Vec2::new(16.0, 10.0);

            ctx.set_style(style);

            Ok(Box::new(app::CofferApp::default()))
        }),
    )
    .unwrap();
}
