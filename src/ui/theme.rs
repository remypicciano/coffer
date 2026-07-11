use eframe::egui::{self, Color32, CornerRadius, Stroke};

// Deep navy foundation
pub const BACKGROUND: Color32 = Color32::from_rgb(8, 13, 27);

pub const SURFACE: Color32 = Color32::from_rgb(16, 24, 43);

pub const SURFACE_RAISED: Color32 = Color32::from_rgb(23, 34, 59);

pub const BORDER: Color32 = Color32::from_rgb(46, 60, 91);

// Brand accents
pub const PRIMARY: Color32 = Color32::from_rgb(103, 99, 255);

pub const PRIMARY_HOVER: Color32 = Color32::from_rgb(124, 120, 255);

pub const ACCENT: Color32 = Color32::from_rgb(78, 181, 255);

// State colors
pub const SUCCESS: Color32 = Color32::from_rgb(70, 211, 138);

pub const WARNING: Color32 = Color32::from_rgb(255, 190, 92);

pub const DANGER: Color32 = Color32::from_rgb(239, 83, 110);

// Text
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(244, 247, 255);

pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(159, 171, 198);

pub const TEXT_MUTED: Color32 = Color32::from_rgb(111, 125, 155);

// Layout
pub const CARD_RADIUS: u8 = 18;
pub const BUTTON_RADIUS: u8 = 12;

pub const STANDARD_PADDING: f32 = 22.0;
pub const SECTION_SPACING: f32 = 16.0;
pub const MAX_CONTENT_WIDTH: f32 = 720.0;

pub fn card_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, BORDER))
        .corner_radius(CornerRadius::same(CARD_RADIUS))
        .inner_margin(STANDARD_PADDING)
}

pub fn raised_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(SURFACE_RAISED)
        .stroke(Stroke::new(1.0_f32, BORDER))
        .corner_radius(CornerRadius::same(CARD_RADIUS))
        .inner_margin(STANDARD_PADDING)
}
