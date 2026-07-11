use eframe::egui::{
    self,
    Color32,
    CornerRadius,
    Stroke,
};


//
// Coffer Color Palette
//

pub const BACKGROUND: Color32 =
    Color32::from_rgb(15, 17, 21);

pub const SURFACE: Color32 =
    Color32::from_rgb(23, 26, 33);

pub const BORDER: Color32 =
    Color32::from_rgb(45, 50, 62);


pub const PRIMARY: Color32 =
    Color32::from_rgb(79, 140, 255);

pub const SUCCESS: Color32 =
    Color32::from_rgb(74, 222, 128);

pub const WARNING: Color32 =
    Color32::from_rgb(251, 191, 36);

pub const DANGER: Color32 =
    Color32::from_rgb(239, 68, 68);


pub const TEXT_PRIMARY: Color32 =
    Color32::from_rgb(248, 250, 252);

pub const TEXT_SECONDARY: Color32 =
    Color32::from_rgb(156, 163, 175);



//
// Layout constants
//

pub const CARD_RADIUS: u8 = 18;

pub const STANDARD_PADDING: f32 = 20.0;

pub const SECTION_SPACING: f32 = 16.0;


//
// Reusable card style
//

pub fn card_frame() -> egui::Frame {

    egui::Frame::new()
        .fill(SURFACE)
        .stroke(
            Stroke::new(
                1.0,
                BORDER
            )
        )
        .corner_radius(
            CornerRadius::same(
                CARD_RADIUS
            )
        )
        .inner_margin(
            STANDARD_PADDING
        )

}
