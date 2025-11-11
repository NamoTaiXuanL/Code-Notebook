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
    use egui::{FontId, TextStyle, Color32};

    style.text_styles.insert(TextStyle::Body, FontId::new(14.0, egui::FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Monospace, FontId::new(14.0, egui::FontFamily::Monospace));
    style.text_styles.insert(TextStyle::Heading, FontId::new(18.0, egui::FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Button, FontId::new(14.0, egui::FontFamily::Proportional));

    // 添加目录列表的字体样式
    style.text_styles.insert(TextStyle::Small, FontId::new(13.0, egui::FontFamily::Proportional));

    // 设置明亮的前景色，提高可读性
    style.visuals.text_color = Color32::from_rgb(240, 240, 240); // 很浅的灰色，接近白色

    ctx.set_style(style);
}

/// 应用状态
struct AppState {
    code: String,           // 代码内容
    file_path: Option<PathBuf>, // 文件路径
    status: String,        // 状态信息
    directory_files: Vec<String>, // 目录文件列表
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            code: "将代码文件拖拽到窗口即可查看".to_string(),
            file_path: None,
            status: String::new(),
            directory_files: Vec::new(),
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 处理拖拽文件
        self.handle_dropped_files(ctx);

        // 更新窗口标题（通过修改窗口的配置）
        if let Some(path) = &self.file_path {
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!("代码查看器 - {}", name_str)));
                }
            }
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title("代码查看器".to_string()));
        }

        // 创建主布局：左侧代码区域 + 右侧目录
        egui::CentralPanel::default().show(ctx, |ui| {
            // 显示状态栏
            if let Some(path) = &self.file_path {
                ui.horizontal(|ui| {
                    ui.label(format!("文件: {}", path.display()));
                    if !self.status.is_empty() {
                        ui.label(format!("状态: {}", self.status));
                    }
                });
                ui.separator();
            }

            // 获取剩余可用空间
            let available_height = ui.available_height();

            // 创建水平布局：代码显示区和目录面板
            ui.horizontal(|ui| {
                // 左侧代码显示区域 - 占75%宽度
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width() * 0.75);
                    ui.set_min_height(available_height);

                    // 代码显示区域 - 使用TextEdit支持编辑
                    egui::ScrollArea::vertical()
                        .id_source("code_content")
                        .auto_shrink([false, false])
                        .stick_to_bottom(false)
                        .show(ui, |ui| {
                            // 确保使用全宽
                            ui.set_width(ui.available_width());

                            // 使用TextEdit显示代码，支持编辑
                            ui.add(
                                egui::TextEdit::multiline(&mut self.code)
                                    .font(egui::TextStyle::Monospace)
                                    .code_editor()
                                    .desired_width(ui.available_width())
                                    .lock_focus(false)
                                    .interactive(true) // 启用编辑
                            );
                        });
                });

                // 右侧目录面板 - 占25%宽度
                ui.separator();

                ui.vertical(|ui| {
                    ui.set_width(ui.available_width());
                    ui.set_min_height(available_height);

                    ui.label("📁 目录");
                    ui.separator();

                    // 目录显示区域 - 独立滚动
                    egui::ScrollArea::vertical()
                        .id_source("file_list") // 设置唯一ID
                        .auto_shrink([false, false])
                        .stick_to_bottom(false)
                        .show(ui, |ui| {
                            // 如果目录为空，显示提示
                            if self.directory_files.is_empty() {
                                ui.add_space(20.0);
                                ui.label("无文件");
                            } else {
                                // 创建一个要加载的文件路径的临时列表
                                let mut file_to_load: Option<PathBuf> = None;

                                // 高亮显示当前文件
                                for file_name in self.directory_files.iter() {
                                    let is_current_file = if let Some(current_path) = &self.file_path {
                                        current_path.file_name()
                                            .and_then(|name| name.to_str())
                                            .map(|current_name| current_name == *file_name)
                                            .unwrap_or(false)
                                    } else {
                                        false
                                    };

                                    // 添加图标
                                    let icon = if file_name.ends_with(".rs") { "🦀 " }
                                        else if file_name.ends_with(".py") { "🐍 " }
                                        else if file_name.ends_with(".js") { "🟨 " }
                                        else if file_name.ends_with(".html") || file_name.ends_with(".htm") { "🌐 " }
                                        else if file_name.ends_with(".css") { "🎨 " }
                                        else if file_name.ends_with(".json") || file_name.ends_with(".xml") { "📄 " }
                                        else if file_name.ends_with(".md") { "📝 " }
                                        else if file_name.ends_with(".gitignore") || file_name.starts_with('.') { "⚙️ " }
                                        else { "📄 " };

                                    let display_name = format!("{}{}", icon, file_name);

                                    if ui.selectable_label(is_current_file, display_name).clicked() {
                                        // 点击目录中的文件时记录要加载的文件
                                        if let Some(current_path) = &self.file_path {
                                            if let Some(parent_dir) = current_path.parent() {
                                                file_to_load = Some(parent_dir.join(file_name));
                                            }
                                        }
                                    }
                                }

                                // 在循环结束后加载文件，避免借用冲突
                                if let Some(new_file_path) = file_to_load {
                                    self.load_file(new_file_path);
                                }
                            }
                        });
                });
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
                self.file_path = Some(path.clone());

                // 加载同目录下的其他文件
                self.load_directory_files(&path);
            }
            Err(e) => {
                self.code = format!("读取失败: {}", e);
                self.status = "错误".to_string();
                self.file_path = None;
                self.directory_files.clear();
            }
        }
    }

    /// 加载目录文件列表
    fn load_directory_files(&mut self, file_path: &PathBuf) {
        self.directory_files.clear();

        if let Some(parent_dir) = file_path.parent() {
            if let Ok(entries) = std::fs::read_dir(parent_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    // 只添加文件，不添加目录
                    if path.is_file() {
                        if let Some(file_name) = path.file_name() {
                            if let Some(name_str) = file_name.to_str() {
                                self.directory_files.push(name_str.to_string());
                            }
                        }
                    }
                }
                // 按字母顺序排序
                self.directory_files.sort();
            }
        }
    }
}