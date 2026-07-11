use eframe::egui;

use crate::ui::theme;



//
// Generic Coffer card
//

pub fn coffer_card(
    ui: &mut egui::Ui,
    title: &str,
    content: impl FnOnce(&mut egui::Ui),
) {

    theme::card_frame()
        .show(
            ui,
            |ui| {

                ui.heading(
                    egui::RichText::new(title)
                        .color(theme::TEXT_PRIMARY)
                );


                ui.add_space(8.0);


                content(ui);

            }
        );

}



//
// Main action button
//

pub fn primary_button(
    ui: &mut egui::Ui,
    text: &str,
) -> egui::Response {


    ui.add_sized(
        [
            ui.available_width(),
            45.0
        ],

        egui::Button::new(
            egui::RichText::new(text)
                .size(18.0)
                .color(
                    egui::Color32::WHITE
                )
        )
        .fill(
            theme::PRIMARY
        )

    )

}



//
// Status indicator
//

pub fn status_pill(
    ui: &mut egui::Ui,
    status: &str,
) {


    let color =
        if status.contains("success")
        || status.contains("Successful")
        {
            theme::SUCCESS
        }

        else if status.contains("error")
        || status.contains("Missing")
        {
            theme::DANGER
        }

        else if status.contains("Decrypt")
        {
            theme::WARNING
        }

        else
        {
            theme::PRIMARY
        };


    ui.horizontal(
        |ui| {

            ui.colored_label(
                color,
                "●"
            );


            ui.label(
                egui::RichText::new(status)
                    .color(
                        theme::TEXT_PRIMARY
                    )
            );

        }
    );

}



//
// Footer
//

pub fn footer(
    ui: &mut egui::Ui
) {


    ui.add_space(20.0);


    ui.vertical_centered(
        |ui| {

            ui.label(
                egui::RichText::new(
                    "Designed & Developed by John Doe"
                )
                .small()
                .color(
                    theme::TEXT_SECONDARY
                )
            );

        }
    );

}
