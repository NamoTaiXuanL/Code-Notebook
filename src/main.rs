// 编码：UTF-8
// 作者：code_notebook项目组Seraphiel

use eframe::egui;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();

    // 初始状态
    let mut initial_state = AppState::default();

    // 如果有命令行参数，尝试作为文件路径加载
    if args.len() > 1 {
        let file_path = std::path::PathBuf::from(&args[1]);
        initial_state.load_file(file_path);
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("代码查看器"),
        ..Default::default()
    };

    eframe::run_native(
        "代码查看器",
        options,
        Box::new(|cc| {
            // 设置中文字体支持
            setup_chinese_fonts(&cc.egui_ctx);
            Box::new(initial_state)
        }),
    )
}

/// 设置中文字体支持
fn setup_chinese_fonts(ctx: &egui::Context) {
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
    use egui::{FontId, TextStyle};

    style.text_styles.insert(TextStyle::Body, FontId::new(16.0, egui::FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Monospace, FontId::new(14.0, egui::FontFamily::Monospace));
    style.text_styles.insert(TextStyle::Heading, FontId::new(20.0, egui::FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Button, FontId::new(16.0, egui::FontFamily::Proportional));
    ctx.set_style(style);
}

/// 应用状态
struct AppState {
    code: String,           // 代码内容
    file_path: Option<PathBuf>, // 文件路径
    status: String,        // 状态信息
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            code: "将代码文件拖拽到窗口即可查看".to_string(),
            file_path: None,
            status: String::new(),
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 处理拖拽文件
        self.handle_dropped_files(ctx);

        // 绘制界面
        egui::CentralPanel::default().show(ctx, |ui| {
            // 显示文件路径和状态
            if let Some(path) = &self.file_path {
                ui.horizontal(|ui| {
                    ui.label(format!("文件: {}", path.display()));
                    if !self.status.is_empty() {
                        ui.label(format!("状态: {}", self.status));
                    }
                });
            }

            // 代码显示区域 - 填满整个可用空间
            egui::ScrollArea::both()
                .auto_shrink([false, false]) // 禁用自动收缩
                .show(ui, |ui| {
                    // 使用可用宽度填满整个区域
                    let available_width = ui.available_width();
                    ui.add(
                        egui::TextEdit::multiline(&mut self.code)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_width(available_width), // 填满宽度
                    );
                });
        });
    }
}

impl AppState {
    /// 处理拖拽文件
    fn handle_dropped_files(&mut self, ctx: &egui::Context) {
        let dropped = ctx.input(|i| i.raw.dropped_files.clone());
        for df in dropped {
            if let Some(path) = df.path {
                self.load_file(path);
            } else if let Some(bytes) = df.bytes {
                // 直接处理字节内容
                self.code = String::from_utf8_lossy(&bytes).into_owned();
                self.status = "已加载临时数据".to_string();
                self.file_path = None;
            }
        }
    }

    /// 加载文件
    fn load_file(&mut self, path: PathBuf) {
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                self.code = content;
                self.status = "已加载".to_string();
                self.file_path = Some(path);
            }
            Err(e) => {
                self.code = format!("读取失败: {}", e);
                self.status = "错误".to_string();
                self.file_path = None;
            }
        }
    }
}