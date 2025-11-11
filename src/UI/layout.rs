use eframe::egui;
use std::path::PathBuf;
use crate::ui::components::{FileBrowser, CodeEditor, StatusBar, SettingsPanel};
use crate::AppState;

/// 主布局管理器
pub struct MainLayout {
    pub file_browser: FileBrowser,
    pub code_editor: CodeEditor,
    pub status_bar: StatusBar,
    pub settings_panel: SettingsPanel,
}

impl MainLayout {
    pub fn new(app_state: &AppState) -> Self {
        let mut file_browser = FileBrowser::new(app_state.current_directory.clone());
        file_browser.directory_items = app_state.directory_items.clone();

        Self {
            file_browser,
            code_editor: CodeEditor::new(app_state.code.clone()),
            status_bar: StatusBar::new(app_state.file_path.clone(), app_state.status.clone()),
            settings_panel: SettingsPanel::new(),
        }
    }

    /// 渲染主布局
    pub fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, app_state: &mut AppState) -> Option<PathBuf> {
        // 只在需要时更新组件状态
        if self.file_browser.current_directory != app_state.current_directory {
            self.file_browser.current_directory = app_state.current_directory.clone();
        }
        if self.file_browser.directory_items.len() != app_state.directory_items.len() {
            self.file_browser.directory_items = app_state.directory_items.clone();
        }
        if self.code_editor.code != app_state.code {
            self.code_editor.code = app_state.code.clone();
        }
        if self.status_bar.file_path != app_state.file_path {
            self.status_bar.file_path = app_state.file_path.clone();
            self.status_bar.status = app_state.status.clone();
        }

        // 更新窗口标题
        self.update_window_title(ctx, &app_state.file_path);

        // 渲染主面板
        egui::CentralPanel::default().show(ctx, |ui| {
            // 获取总可用高度，不受状态栏影响
            let total_available_height = ui.available_height();

            // 渲染状态栏（如果有文件）
            if app_state.file_path.is_some() {
                self.status_bar.render(ui);
            }

            // 计算剩余可用高度给内容区域
            let remaining_height = ui.available_height();

            // 创建水平布局：代码显示区和目录面板
            ui.horizontal(|ui| {
                // 左侧代码显示区域 - 占75%宽度
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width() * 0.75);
                    ui.set_min_height(remaining_height);

                    self.code_editor.render(ui, remaining_height);
                });

                // 右侧目录面板 - 占25%宽度
                ui.separator();

                let file_to_load = if app_state.show_settings {
                    // 显示设置面板 - 不返回文件路径
                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width());
                        ui.set_min_height(remaining_height);
                        self.settings_panel.render(ui, remaining_height, &mut app_state.show_settings);
                    });
                    None
                } else {
                    // 显示文件浏览器 - 可能返回文件路径
                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width());
                        ui.set_min_height(remaining_height);
                        self.file_browser.render(ui, &app_state.file_path, remaining_height, &mut app_state.show_settings)
                    }).inner
                };

                file_to_load
            }).inner
        }).inner
    }

    /// 更新窗口标题
    fn update_window_title(&self, ctx: &egui::Context, file_path: &Option<PathBuf>) {
        if let Some(path) = file_path {
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!("{} - 代码查看器", name_str)));
                }
            }
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title("代码查看器".to_string()));
        }
    }
}