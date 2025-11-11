use eframe::egui;
use std::path::PathBuf;
use crate::ui::components::{FileBrowser, CodeEditor, StatusBar};
use crate::AppState;

/// 主布局管理器
pub struct MainLayout {
    pub file_browser: FileBrowser,
    pub code_editor: CodeEditor,
    pub status_bar: StatusBar,
}

impl MainLayout {
    pub fn new(app_state: &AppState) -> Self {
        Self {
            file_browser: FileBrowser::new(app_state.current_directory.clone()),
            code_editor: CodeEditor::new(app_state.code.clone()),
            status_bar: StatusBar::new(app_state.file_path.clone(), app_state.status.clone()),
        }
    }

    /// 渲染主布局
    pub fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, app_state: &mut AppState) -> Option<PathBuf> {
        // 更新组件状态
        self.file_browser.current_directory = app_state.current_directory.clone();
        self.file_browser.directory_items = app_state.directory_items.clone();
        self.code_editor.code = app_state.code.clone();
        self.status_bar.file_path = app_state.file_path.clone();
        self.status_bar.status = app_state.status.clone();

        // 更新窗口标题
        self.update_window_title(ctx, &app_state.file_path);

        // 渲染主面板
        egui::CentralPanel::default().show(ctx, |ui| {
            // 渲染状态栏
            let status_height = if app_state.file_path.is_some() {
                self.status_bar.render(ui);
                ui.available_height()
            } else {
                ui.available_height()
            };

            // 创建水平布局：代码显示区和目录面板
            ui.horizontal(|ui| {
                // 左侧代码显示区域 - 占75%宽度
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width() * 0.75);
                    ui.set_min_height(status_height);

                    self.code_editor.render(ui, status_height);
                });

                // 右侧目录面板 - 占25%宽度
                ui.separator();

                let file_to_load = ui.vertical(|ui| {
                    ui.set_width(ui.available_width());
                    ui.set_min_height(status_height);

                    self.file_browser.render(ui, &app_state.file_path, status_height)
                }).inner;


                // 如果需要，更新app状态
                if let Some(new_code) = Some(self.code_editor.code.clone()) {
                    if new_code != app_state.code {
                        app_state.code = new_code;
                    }
                }

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