use eframe::egui;
use rfd::FileDialog;

use std::path::PathBuf;



pub struct CofferApp {

    pub encrypted_file: Option<PathBuf>,
    pub key_file: Option<PathBuf>,
    pub status: String,
    pub progress: f32,
    pub show_success: bool,
    pub show_error: bool,
    pub show_viewer: bool,
    pub decrypted_text: Option<String>,

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





    pub fn select_file(&mut self) {


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





    pub fn select_key(&mut self) {


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






    pub fn decrypt(&mut self) {


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

        crate::ui::home::show(
            self,
            ctx
        );

    }

}
