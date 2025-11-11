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
    ) -> Option<PathBuf> {
        let mut file_to_load: Option<PathBuf> = None;
        let mut directory_to_enter: Option<PathBuf> = None;

        ui.set_width(ui.available_width());
        ui.set_min_height(available_height);

        // 固定的返回上级目录按钮 - 直接在顶部
        if self.current_directory.parent().is_some() {
            if ui.selectable_label(false, "⬆️ .. 返回上级目录").clicked() {
                if let Some(parent) = self.current_directory.parent() {
                    directory_to_enter = Some(parent.to_path_buf());
                }
            }
            ui.separator();
        }

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

/// 代码编辑器组件
pub struct CodeEditor {
    pub code: String,
}

impl CodeEditor {
    pub fn new(code: String) -> Self {
        Self { code }
    }

    /// 渲染代码编辑器
    pub fn render(&mut self, ui: &mut egui::Ui, available_height: f32) {
        ui.set_width(ui.available_width());
        ui.set_min_height(available_height);

        // 代码显示区域 - 使用TextEdit支持编辑
        egui::ScrollArea::vertical()
            .id_source("code_content")
            .auto_shrink([false, false])
            .stick_to_bottom(false)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                ui.add(
                    egui::TextEdit::multiline(&mut self.code)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_width(ui.available_width())
                        .lock_focus(false)
                        .interactive(true)
                );
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