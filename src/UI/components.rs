use eframe::egui;
use std::path::PathBuf;
use crate::DirectoryItem;

/// 文件管理器组件
pub struct FileBrowser {
    pub current_directory: PathBuf,
    pub directory_items: Vec<DirectoryItem>,
}

impl FileBrowser {
    pub fn new(current_directory: PathBuf) -> Self {
        Self {
            current_directory,
            directory_items: Vec::new(),
        }
    }

    /// 渲染文件浏览器
    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        file_path: &Option<PathBuf>,
        available_height: f32,
        show_settings: &mut bool,
    ) -> Option<PathBuf> {
        let mut file_to_load: Option<PathBuf> = None;
        let mut directory_to_enter: Option<PathBuf> = None;

        ui.set_width(ui.available_width());
        ui.set_min_height(available_height);

        // 顶部按钮区域
        ui.horizontal(|ui| {
            // 返回上级目录按钮
            if self.current_directory.parent().is_some() {
                if ui.selectable_label(false, ".. 返回上级").clicked() {
                    if let Some(parent) = self.current_directory.parent() {
                        directory_to_enter = Some(parent.to_path_buf());
                    }
                }
            }

            // 设置按钮
            let settings_text = if *show_settings { "[设置] " } else { "设置" };
            if ui.selectable_label(*show_settings, settings_text).clicked() {
                *show_settings = !*show_settings;
            }
        });
        ui.separator();

        // 目录显示区域 - 使用剩余空间
        egui::ScrollArea::vertical()
            .id_source("file_list")
            .auto_shrink([false, false])
            .stick_to_bottom(false)
            .show(ui, |ui| {
                if self.directory_items.is_empty() {
                    ui.add_space(20.0);
                    ui.label("目录为空");
                } else {
                    // 高亮显示当前文件
                    for item in &self.directory_items {
                        let is_current_file = if !item.is_directory {
                            if let Some(current_path) = file_path {
                                current_path.file_name()
                                    .and_then(|name| name.to_str())
                                    .map(|current_name| current_name == item.name)
                                    .unwrap_or(false)
                            } else {
                                false
                            }
                        } else {
                            false
                        };

                        // 添加图标
                        let icon = if item.is_directory {
                            "📁 "
                        } else if item.name.ends_with(".rs") { "🦀 " }
                            else if item.name.ends_with(".py") { "🐍 " }
                            else if item.name.ends_with(".js") { "🟨 " }
                            else if item.name.ends_with(".html") || item.name.ends_with(".htm") { "🌐 " }
                            else if item.name.ends_with(".css") { "🎨 " }
                            else if item.name.ends_with(".json") || item.name.ends_with(".xml") { "📄 " }
                            else if item.name.ends_with(".md") { "📝 " }
                            else if item.name.ends_with(".gitignore") || item.name.starts_with('.') { "⚙️ " }
                            else { "📄 " };

                        let display_name = format!("{}{}", icon, item.name);

                        if ui.selectable_label(is_current_file, display_name).clicked() {
                            if item.is_directory {
                                directory_to_enter = Some(item.path.clone());
                            } else {
                                file_to_load = Some(item.path.clone());
                            }
                        }
                    }
                }
            });

        // 处理目录切换
        if let Some(dir_path) = directory_to_enter {
            self.current_directory = dir_path;
            self.load_directory_content();
        }

        file_to_load
    }

    /// 加载当前目录的内容
    pub fn load_directory_content(&mut self) {
        self.directory_items.clear();

        if let Ok(entries) = std::fs::read_dir(&self.current_directory) {
            let mut directories = Vec::new();
            let mut files = Vec::new();

            for entry in entries.flatten() {
                let path = entry.path();
                let is_directory = path.is_dir();

                if let Some(file_name) = path.file_name() {
                    if let Some(name_str) = file_name.to_str() {
                        let item = DirectoryItem {
                            name: name_str.to_string(),
                            path: path.clone(),
                            is_directory,
                        };

                        if is_directory {
                            directories.push(item);
                        } else {
                            files.push(item);
                        }
                    }
                }
            }

            // 排序：目录在前，文件在后，都按字母顺序排序
            directories.sort_by(|a, b| a.name.cmp(&b.name));
            files.sort_by(|a, b| a.name.cmp(&b.name));

            // 合并列表
            self.directory_items.extend(directories);
            self.directory_items.extend(files);
        }
    }
}

use crate::ui::syntax_highlighter::SyntaxHighlighter;

/// 代码编辑器组件
pub struct CodeEditor {
    pub code: String,
    syntax_highlighter: SyntaxHighlighter,
    show_syntax_highlighting: bool, // true = 语法高亮只读, false = 编辑模式
    scroll_offset_y: f32,
}

impl CodeEditor {
    pub fn new(code: String) -> Self {
        Self {
            code,
            syntax_highlighter: SyntaxHighlighter::new(),
            show_syntax_highlighting: false, // 默认编辑模式
            scroll_offset_y: 0.0,
        }
    }

    /// 渲染代码编辑器
    pub fn render(&mut self, ui: &mut egui::Ui, available_height: f32) {
        ui.set_width(ui.available_width());
        ui.set_min_height(available_height);

        // 代码显示区域 - 添加双向滚动
        egui::ScrollArea::both()
            .id_source("code_content")
            .auto_shrink([false, false])
            .stick_to_bottom(false)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                // 简化方案：优先使用纯编辑模式
                if !self.show_syntax_highlighting {
                    // 普通编辑模式 - 无语法高亮，性能最佳
                    ui.add(
                        egui::TextEdit::multiline(&mut self.code)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .lock_focus(false)
                            .interactive(true)
                    );
                } else {
                    // 语法高亮只读模式 - 避免在编辑器中使用 layouter
                    let mut layout_jobs = Vec::new();
                    for (line_idx, line) in self.code.lines().enumerate() {
                        let mut job = egui::text::LayoutJob::default();

                        // 添加行号
                        job.append(
                            &format!("{:>4} ", line_idx + 1),
                            0.0,
                            egui::TextFormat {
                                font_id: egui::FontId::monospace(12.0),
                                color: egui::Color32::GRAY,
                                ..Default::default()
                            },
                        );

                        // 添加语法高亮的代码行
                        let tokens = self.syntax_highlighter.parse_line_public(line);
                        for token in tokens {
                            job.append(
                                &token.text,
                                0.0,
                                egui::TextFormat {
                                    font_id: egui::FontId::monospace(12.0),
                                    color: token.color,
                                    ..Default::default()
                                },
                            );
                        }

                        layout_jobs.push(job);
                    }

                    // 显示只读的语法高亮文本
                    ui.vertical(|ui| {
                        for job in layout_jobs {
                            ui.add(egui::Label::new(job));
                        }
                    });
                }
            });

        // 添加模式切换按钮
        ui.horizontal(|ui| {
            if ui.button(if self.show_syntax_highlighting { "切换到编辑模式" } else { "切换到语法高亮" }).clicked() {
                self.show_syntax_highlighting = !self.show_syntax_highlighting;
            }

            ui.label(if self.show_syntax_highlighting {
                "✨ 语法高亮模式（只读）"
            } else {
                "⚡ 编辑模式（可修改）"
            });
        });
    }
}

/// 状态栏组件
pub struct StatusBar {
    pub file_path: Option<PathBuf>,
    pub status: String,
}

impl StatusBar {
    pub fn new(file_path: Option<PathBuf>, status: String) -> Self {
        Self { file_path, status }
    }

    /// 渲染状态栏
    pub fn render(&mut self, ui: &mut egui::Ui) {
        if let Some(path) = &self.file_path {
            ui.horizontal(|ui| {
                ui.label(format!("文件: {}", path.display()));
                if !self.status.is_empty() {
                    ui.label(format!("状态: {}", self.status));
                }
            });
            ui.separator();
        }
    }
}

/// 设置框组件
pub struct SettingsPanel;

impl SettingsPanel {
    pub fn new() -> Self {
        Self
    }

    /// 渲染设置面板
    pub fn render(&mut self, ui: &mut egui::Ui, available_height: f32, show_settings: &mut bool) {
        ui.set_width(ui.available_width());
        ui.set_min_height(available_height);

        // 顶部返回按钮
        if ui.selectable_label(false, "返回文件列表").clicked() {
            *show_settings = false;
        }

        // 简单的设置面板
        ui.heading("界面设置");
        ui.separator();

        ui.add_space(20.0);
        ui.label("设置功能开发中...");
    }
}