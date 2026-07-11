mod crypto;
mod key;
mod paths;
mod app;


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

            ctx.set_visuals(
                egui::Visuals::dark()
            );


            Ok(Box::new(
                app::CofferApp::default()
            ))

        }),
    )
    .unwrap();

}