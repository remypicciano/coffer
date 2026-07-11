mod app;
mod crypto;
mod key;
mod paths;
mod ui;


fn main() {

    let options = eframe::NativeOptions {

        viewport:
            egui::ViewportBuilder::default()
            .with_inner_size([520.0, 680.0])
            .with_min_inner_size([450.0, 550.0]),

        ..Default::default()
    };


    eframe::run_native(
        "Coffer",
        options,

        Box::new(|cc| {

            let ctx = &cc.egui_ctx;

            let mut visuals = egui::Visuals::dark();

            visuals.panel_fill = ui::theme::BACKGROUND;

            visuals.window_fill = ui::theme::BACKGROUND;

            visuals.selection.bg_fill =
                ui::theme::PRIMARY;

            visuals.selection.stroke.color =
                egui::Color32::WHITE;

            ctx.set_visuals(visuals);


            Ok(Box::new(
                app::CofferApp::default()
            ))

        }),
    )
    .unwrap();

}
