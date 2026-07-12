mod app;
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

            ui::theme::apply_visuals(ctx, true);

            let mut style = (*ctx.style()).clone();

            style.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);

            style.spacing.button_padding = egui::Vec2::new(16.0, 10.0);

            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::new(15.0, egui::FontFamily::Proportional),
            );
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            );
            style.text_styles.insert(
                egui::TextStyle::Small,
                egui::FontId::new(12.0, egui::FontFamily::Proportional),
            );

            ctx.set_style(style);

            Ok(Box::new(app::CofferApp::default()))
        }),
    )
    .unwrap();
}
