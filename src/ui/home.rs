use eframe::egui;

use crate::app::CofferApp;

use crate::ui::{
    theme,
    widgets,
};



pub fn show(
    app: &mut CofferApp,
    ctx: &egui::Context,
)
{

    egui::CentralPanel::default()
        .show(
            ctx,
            |ui|
            {

                ui.vertical_centered(
                    |ui|
                    {

                        ui.add_space(25.0);


                        ui.heading(
                            egui::RichText::new(
                                "🥐"
                            )
                            .size(48.0)
                        );


                        ui.heading(
                            egui::RichText::new(
                                "COFFER"
                            )
                            .size(34.0)
                            .color(
                                theme::TEXT_PRIMARY
                            )
                        );


                        ui.label(
                            egui::RichText::new(
                                "Secure • Local • Private"
                            )
                            .color(
                                theme::TEXT_SECONDARY
                            )
                        );


                    }
                );


                ui.add_space(35.0);



                widgets::coffer_card(
                    ui,
                    "Encrypted File",
                    |ui|
                    {

                        if ui.button(
                            "Choose encrypted file"
                        )
                        .clicked()
                        {

                            app.select_file();

                        }


                        if let Some(path) =
                            &app.encrypted_file
                        {

                            ui.add_space(8.0);


                            ui.label(
                                path.display()
                                .to_string()
                            );

                        }

                    }
                );



                ui.add_space(
                    theme::SECTION_SPACING
                );



                widgets::coffer_card(
                    ui,
                    "Encryption Key",
                    |ui|
                    {

                        if ui.button(
                            "Choose key"
                        )
                        .clicked()
                        {

                            app.select_key();

                        }


                        if let Some(path) =
                            &app.key_file
                        {

                            ui.add_space(8.0);


                            ui.label(
                                path.display()
                                .to_string()
                            );

                        }

                    }
                );



                ui.add_space(
                    theme::SECTION_SPACING
                );



                widgets::coffer_card(
                    ui,
                    "Status",
                    |ui|
                    {

                        widgets::status_pill(
                            ui,
                            &app.status
                        );


                        ui.add_space(8.0);


                        ui.add(
                            egui::ProgressBar::new(
                                app.progress
                            )
                        );


                    }
                );



                ui.add_space(25.0);



                if widgets::primary_button(
                    ui,
                    "🔓 Decrypt File"
                )
                .clicked()
                {

                    app.decrypt();

                }



                widgets::footer(ui);


            }
        );

}
