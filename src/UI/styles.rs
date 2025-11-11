use eframe::egui;
use egui::{FontId, TextStyle, Color32};

/// 设置中文字体支持和应用样式
pub fn setup_chinese_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 尝试加载微软雅黑字体
    let font_data = std::fs::read("C:\\Windows\\Fonts\\msyh.ttc");

    if let Ok(font_bytes) = font_data {
        fonts.font_data.insert(
            "chinese_font".to_owned(),
            egui::FontData::from_owned(font_bytes)
        );

        // 设置为默认字体
        fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "chinese_font".to_owned());
        fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "chinese_font".to_owned());
    }

    ctx.set_fonts(fonts);

    // 设置字体大小和样式
    let mut style = (*ctx.style()).clone();

    style.text_styles.insert(TextStyle::Body, FontId::new(14.0, egui::FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Monospace, FontId::new(14.0, egui::FontFamily::Monospace));
    style.text_styles.insert(TextStyle::Heading, FontId::new(18.0, egui::FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Button, FontId::new(14.0, egui::FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Small, FontId::new(13.0, egui::FontFamily::Proportional));

    // 设置更明亮的前景色，提高可读性
    style.visuals.widgets.noninteractive.fg_stroke.color = Color32::from_rgb(240, 240, 240);
    style.visuals.selection.stroke.color = Color32::from_rgb(100, 150, 255);
    style.visuals.selection.bg_fill = Color32::from_rgba_premultiplied(100, 150, 255, 50);

    ctx.set_style(style);
}