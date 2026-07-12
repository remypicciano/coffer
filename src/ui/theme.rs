use std::sync::atomic::{AtomicBool, Ordering};

use eframe::egui::{self, Color32};

static LIGHT_MODE: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy)]
struct Palette {
    background: Color32,
    surface: Color32,
    surface_raised: Color32,
    border: Color32,
    primary: Color32,
    primary_hover: Color32,
    on_primary: Color32,
    success: Color32,
    warning: Color32,
    danger: Color32,
    text_primary: Color32,
    text_secondary: Color32,
    text_muted: Color32,
}

const DARK: Palette = Palette {
    background: Color32::from_rgb(6, 9, 17),
    surface: Color32::from_rgb(17, 19, 29),
    surface_raised: Color32::from_rgb(29, 30, 43),
    border: Color32::from_rgb(55, 56, 73),
    primary: Color32::from_rgb(144, 131, 255),
    primary_hover: Color32::from_rgb(165, 154, 255),
    on_primary: Color32::from_rgb(10, 9, 19),
    success: Color32::from_rgb(197, 255, 0),
    warning: Color32::from_rgb(255, 197, 91),
    danger: Color32::from_rgb(255, 112, 133),
    text_primary: Color32::from_rgb(250, 249, 255),
    text_secondary: Color32::from_rgb(203, 201, 218),
    text_muted: Color32::from_rgb(145, 143, 163),
};

const LIGHT: Palette = Palette {
    background: Color32::from_rgb(243, 241, 239),
    surface: Color32::from_rgb(255, 255, 255),
    surface_raised: Color32::from_rgb(236, 233, 244),
    border: Color32::from_rgb(215, 211, 222),
    primary: Color32::from_rgb(101, 82, 230),
    primary_hover: Color32::from_rgb(79, 61, 204),
    on_primary: Color32::from_rgb(255, 255, 255),
    success: Color32::from_rgb(70, 100, 0),
    warning: Color32::from_rgb(154, 91, 0),
    danger: Color32::from_rgb(183, 43, 67),
    text_primary: Color32::from_rgb(6, 9, 17),
    text_secondary: Color32::from_rgb(67, 65, 78),
    text_muted: Color32::from_rgb(105, 102, 116),
};

fn palette() -> Palette {
    if LIGHT_MODE.load(Ordering::Relaxed) {
        LIGHT
    } else {
        DARK
    }
}

pub fn set_light_mode(light: bool) {
    LIGHT_MODE.store(light, Ordering::Relaxed);
}

pub fn apply_visuals(ctx: &egui::Context, light: bool) {
    set_light_mode(light);
    let colors = palette();
    let mut visuals = if light {
        egui::Visuals::light()
    } else {
        egui::Visuals::dark()
    };
    visuals.panel_fill = colors.background;
    visuals.window_fill = colors.surface;
    visuals.extreme_bg_color = colors.background;
    visuals.faint_bg_color = colors.surface_raised;
    visuals.selection.bg_fill = colors.primary;
    visuals.selection.stroke.color = colors.on_primary;
    visuals.widgets.inactive.bg_fill = colors.surface_raised;
    visuals.widgets.inactive.weak_bg_fill = colors.surface_raised;
    visuals.widgets.hovered.bg_fill = colors.primary_hover;
    visuals.widgets.active.bg_fill = colors.primary;
    visuals.widgets.noninteractive.bg_fill = colors.surface;
    visuals.widgets.noninteractive.fg_stroke.color = colors.text_secondary;
    visuals.widgets.inactive.fg_stroke.color = colors.text_primary;
    visuals.widgets.hovered.fg_stroke.color = colors.on_primary;
    visuals.widgets.active.fg_stroke.color = colors.on_primary;
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(10);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(10);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(10);
    visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(10);
    ctx.set_visuals(visuals);
}

macro_rules! color_getters {
    ($($name:ident),+ $(,)?) => { $(pub fn $name() -> Color32 { palette().$name })+ };
}

color_getters!(
    background,
    surface,
    surface_raised,
    border,
    primary,
    primary_hover,
    on_primary,
    success,
    warning,
    danger,
    text_primary,
    text_secondary,
    text_muted
);

#[cfg(test)]
mod tests {
    use super::*;

    fn luminance(color: Color32) -> f32 {
        fn channel(value: u8) -> f32 {
            let value = f32::from(value) / 255.0;
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        }
        0.2126 * channel(color.r()) + 0.7152 * channel(color.g()) + 0.0722 * channel(color.b())
    }

    fn contrast(a: Color32, b: Color32) -> f32 {
        let (lighter, darker) = {
            let a = luminance(a);
            let b = luminance(b);
            if a > b { (a, b) } else { (b, a) }
        };
        (lighter + 0.05) / (darker + 0.05)
    }

    #[test]
    fn both_palettes_meet_text_and_button_contrast_targets() {
        for palette in [LIGHT, DARK] {
            assert!(contrast(palette.text_primary, palette.background) >= 7.0);
            assert!(contrast(palette.on_primary, palette.primary) >= 4.5);
        }
    }
}
