use eframe::egui;
use rfd::FileDialog;

use std::path::PathBuf;



pub struct CofferApp {


    encrypted_file: Option<PathBuf>,

    key_file: Option<PathBuf>,


    status: String,

    progress: f32,


    show_success: bool,

    show_error: bool,


    show_viewer: bool,

    decrypted_text: Option<String>,


}



impl Default for CofferApp {


    fn default() -> Self {

        Self {

            encrypted_file: None,

            key_file: None,


            status: "Ready".into(),

            progress: 0.0,


            show_success: false,

            show_error: false,


            show_viewer: false,

            decrypted_text: None,

        }

    }

}




impl CofferApp {


    fn card(
        ui: &mut egui::Ui,
        title: &str,
        body: impl FnOnce(&mut egui::Ui)
    )
    {

        egui::Frame::group(
            ui.style()
        )
        .inner_margin(16.0)

        .show(ui, |ui| {

            ui.heading(title);

            ui.add_space(8.0);

            body(ui);

        });

    }





    fn select_file(&mut self) {


        if let Some(path) =
            FileDialog::new()
            .add_filter(
                "Coffer encrypted files",
                &["coffer"]
            )
            .pick_file()
        {

            self.encrypted_file =
                Some(path);


            self.status =
                "Encrypted file loaded".into();

        }

    }





    fn select_key(&mut self) {


        if let Some(path) =
            FileDialog::new()
            .add_filter(
                "Encryption key",
                &["bin"]
            )
            .pick_file()
        {

            self.key_file =
                Some(path);


            self.status =
                "Key loaded".into();

        }

    }






    fn decrypt(&mut self) {


        if self.encrypted_file.is_none()
            ||
            self.key_file.is_none()
        {


            self.status =
                "Missing file or key".into();


            self.show_error = true;


            return;

        }



        self.status =
            "Decrypting...".into();


        self.progress = 1.0;



        // Placeholder until crypto connection

        self.decrypted_text =
            Some(
                "Secret decrypted message."
                .into()
            );


        self.status =
            "Decryption successful".into();


        self.show_success = true;


    }





}



impl eframe::App for CofferApp {


    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,

    )
    {


        egui::CentralPanel::default()
        .show(ctx, |ui| {



            ui.vertical_centered(|ui| {


                ui.heading(
                    egui::RichText::new(
                        "🔐 COFFER"
                    )
                    .size(32.0)
                );


                ui.label(
                    "Private file encryption"
                );


            });



            ui.add_space(25.0);




            Self::card(
                ui,
                "Encrypted File",
                |ui| {


                    if ui.button(
                        "Choose encrypted file"
                    )
                    .clicked()
                    {

                        self.select_file();

                    }



                    if let Some(path) =
                        &self.encrypted_file
                    {

                        ui.label(
                            egui::RichText::new(
                                path.display()
                                .to_string()
                            )
                            .weak()
                        );

                    }

                }
            );



            ui.add_space(12.0);





            Self::card(
                ui,
                "Encryption Key",
                |ui| {


                    if ui.button(
                        "Choose key"
                    )
                    .clicked()
                    {

                        self.select_key();

                    }




                    if let Some(path) =
                        &self.key_file
                    {

                        ui.label(
                            egui::RichText::new(
                                path.display()
                                .to_string()
                            )
                            .weak()
                        );

                    }


                }
            );



            ui.add_space(12.0);




            Self::card(
                ui,
                "Status",
                |ui| {


                    ui.label(
                        &self.status
                    );


                    ui.add(
                        egui::ProgressBar::new(
                            self.progress
                        )
                    );


                }
            );



            ui.add_space(20.0);



            let decrypt_button =
                egui::Button::new(
                    egui::RichText::new(
                        "Decrypt"
                    )
                    .size(18.0)

                );



            if ui
                .add_sized(
                    [
                        ui.available_width(),
                        45.0
                    ],
                    decrypt_button
                )
                .clicked()

            {

                self.decrypt();

            }



        });






        // SUCCESS WINDOW

        if self.show_success {


            egui::Window::new(
                "Success"
            )

            .collapsible(false)

            .show(
                ctx,
                |ui| {


                    ui.heading(
                        "✓ Decrypted"
                    );


                    ui.label(
                        "Your file was successfully decrypted."
                    );


                    ui.add_space(15.0);



                    if ui.button(
                        "Open secure viewer"
                    )
                    .clicked()

                    {

                        self.show_viewer = true;

                        self.show_success = false;

                    }


                }
            );


        }





        // SECURE VIEWER

        if self.show_viewer {


            egui::Window::new(
                "Secure Viewer"
            )

            .collapsible(false)

            .resizable(true)

            .show(
                ctx,
                |ui| {


                    ui.colored_label(
                        egui::Color32::RED,

                        "⚠ Closing this window wipes the decrypted content"
                    );


                    ui.separator();



                    if let Some(text) =
                        &self.decrypted_text
                    {


                        ui.add(
                            egui::TextEdit::multiline(
                                &mut text.clone()
                            )
                            .desired_rows(12)
                        );


                    }



                    if ui.button(
                        "Close and wipe"
                    )
                    .clicked()

                    {


                        self.decrypted_text = None;


                        self.show_viewer = false;


                    }


                }
            );


        }





        // ERROR WINDOW

        if self.show_error {


            egui::Window::new(
                "Error"
            )

            .show(
                ctx,
                |ui| {


                    ui.colored_label(
                        egui::Color32::RED,

                        "Missing encrypted file or key."
                    );



                    if ui.button(
                        "Close"
                    )
                    .clicked()
                    {

                        self.show_error = false;

                    }


                }
            );


        }


    }

}