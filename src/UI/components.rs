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
    cached_highlighted_lines: Vec<egui::text::LayoutJob>,
    layout_job_pool: Vec<egui::text::LayoutJob>, // LayoutJob对象内存池
    last_code_hash: u64,
    cached_line_height: f32,
    last_scroll_position: f32, // 上次滚动位置，用于检测滚动状态
    is_scrolling: bool,         // 是否正在滚动
    scroll_timer: u32,          // 滚动计时器，用于延迟语法高亮
}

impl CodeEditor {
    pub fn new(code: String) -> Self {
        let code_hash = Self::calculate_code_hash(&code);
        Self {
            code,
            syntax_highlighter: SyntaxHighlighter::new(),
            show_syntax_highlighting: true, // 默认语法高亮模式
            cached_highlighted_lines: Vec::new(),
            layout_job_pool: Vec::new(), // 初始化内存池
            last_code_hash: code_hash,
            cached_line_height: 0.0,
            last_scroll_position: 0.0,
            is_scrolling: false,
            scroll_timer: 0,
        }
    }

    /// 渲染代码编辑器
    pub fn render(&mut self, ui: &mut egui::Ui, available_height: f32) {
        ui.set_width(ui.available_width());
        ui.set_min_height(available_height);

        // 检测滚动状态
        self.detect_scrolling_state(ui);

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
                    // 语法高亮只读模式 - 只渲染可见区域
                    self.render_visible_syntax_highlighted(ui);
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

    /// 渲染可见区域的语法高亮（超高效版本）
    fn render_visible_syntax_highlighted(&mut self, ui: &mut egui::Ui) {
        // 只在非滚动状态或滚动停止时更新缓存
        if !self.is_scrolling || self.scroll_timer == 0 {
            self.update_cached_lines();
        }

        // 先收集行信息，避免同时借用
        let lines_count = self.code.lines().count();
        if lines_count == 0 || self.cached_highlighted_lines.is_empty() {
            return;
        }

        // 获取行高（使用缓存避免频繁查询字体）
        let line_height = self.get_cached_line_height(ui);

        // 只在滚动时重新计算可见区域（性能优化）
        let (start_line, end_line) = self.calculate_visible_lines(ui, line_height, lines_count);

        // 为顶部空间占位
        let top_space = (start_line as f32) * line_height;
        if top_space > 0.0 {
            ui.add_space(top_space);
        }

        // 只渲染可见区域的行
        for line_idx in start_line..end_line {
            let line_num = line_idx + 1;

            ui.horizontal(|ui| {
                // 行号
                ui.label(
                    egui::RichText::new(format!("{:>4}", line_num))
                        .monospace()
                        .color(egui::Color32::GRAY)
                        .size(12.0)
                );

                // 使用缓存的语法高亮
                if line_idx < self.cached_highlighted_lines.len() {
                    ui.add(egui::Label::new(self.cached_highlighted_lines[line_idx].clone()));
                } else {
                    // 如果缓存中没有该行，显示原始文本（防止内容截断）
                    // 需要重新获取行内容，避免借用冲突
                    if let Some(line_text) = self.code.lines().nth(line_idx) {
                        ui.label(
                            egui::RichText::new(line_text)
                                .monospace()
                                .size(12.0)
                        );
                    }
                }
            });
        }

        // 为底部空间占位（确保滚动条正确工作）
        let bottom_space = ((lines_count - end_line) as f32) * line_height;
        if bottom_space > 0.0 {
            ui.add_space(bottom_space);
        }
    }

    /// 检测滚动状态并更新计时器
    fn detect_scrolling_state(&mut self, ui: &egui::Ui) {
        // 获取当前滚动位置
        let current_scroll = ui.clip_rect().min.y;
        
        // 检测滚动状态变化
        if (current_scroll - self.last_scroll_position).abs() > 1.0 {
            // 正在滚动
            self.is_scrolling = true;
            self.scroll_timer = 5; // 设置计时器为5帧
        } else if self.scroll_timer > 0 {
            // 计时器递减
            self.scroll_timer -= 1;
        } else {
            // 滚动停止
            self.is_scrolling = false;
        }
        
        // 更新上次滚动位置
        self.last_scroll_position = current_scroll;
    }

    /// 更新缓存的语法高亮行（只在代码变化时）
    fn update_cached_lines(&mut self) {
        let current_hash = Self::calculate_code_hash(&self.code);

        // 如果代码没有变化，使用缓存
        if current_hash == self.last_code_hash && !self.cached_highlighted_lines.is_empty() {
            return;
        }

        // 先收集所有需要的信息，避免同时借用
        let old_lines: Vec<String> = if self.last_code_hash != 0 {
            self.code.lines().map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        };
        
        let lines: Vec<String> = self.code.lines().map(|s| s.to_string()).collect();
        self.last_code_hash = current_hash;

        // 如果行数减少，截断缓存并释放内存
        if lines.len() < self.cached_highlighted_lines.len() {
            let removed_jobs = self.cached_highlighted_lines.split_off(lines.len());
            for job in removed_jobs {
                self.return_layout_job_to_pool(job);
            }
        }

        // 收集需要更新的行信息
        let mut lines_to_update = Vec::new();
        
        for (line_idx, line) in lines.iter().enumerate() {
            let needs_update = if line_idx < old_lines.len() {
                // 检查行是否发生变化
                line_idx >= self.cached_highlighted_lines.len() || 
                old_lines.get(line_idx) != Some(line)
            } else {
                // 新行
                true
            };

            if needs_update {
                lines_to_update.push((line_idx, line.clone()));
            }
        }

        // 进行并行处理
        if !lines_to_update.is_empty() {
            self.update_cached_lines_parallel(lines_to_update);
        }
    }

    /// 获取缓存的行高（避免频繁查询字体）
    fn get_cached_line_height(&mut self, ui: &egui::Ui) -> f32 {
        // 如果已经缓存了行高，直接返回
        if self.cached_line_height > 0.0 {
            return self.cached_line_height;
        }
        
        // 计算并缓存行高
        self.cached_line_height = ui.fonts(|fonts| fonts.row_height(&egui::FontId::monospace(12.0)));
        self.cached_line_height
    }

    /// 计算可见行范围（只在滚动时重新计算）
    fn calculate_visible_lines(&mut self, ui: &egui::Ui, line_height: f32, total_lines: usize) -> (usize, usize) {
        // 获取视口信息
        let scroll_area_rect = ui.max_rect();
        let viewport_top = ui.clip_rect().min.y - scroll_area_rect.min.y;
        let viewport_bottom = viewport_top + ui.clip_rect().height();

        // 计算基础可见行范围
        let start_line = ((viewport_top / line_height).floor() as usize).max(0);
        let end_line = ((viewport_bottom / line_height).ceil() as usize).min(total_lines);

        // 添加缓冲区以实现平滑滚动
        let buffer_size = (ui.clip_rect().height() / line_height * 0.3).ceil() as usize; // 减少缓冲区大小
        let start_line = start_line.saturating_sub(buffer_size);
        let end_line = (end_line + buffer_size).min(total_lines);

        (start_line, end_line)
    }

    /// 从内存池获取或创建LayoutJob
    fn get_layout_job_from_pool(&mut self) -> egui::text::LayoutJob {
        if let Some(mut job) = self.layout_job_pool.pop() {
            // 清空重用对象
            job.sections.clear();
            job
        } else {
            // 池为空，创建新对象
            egui::text::LayoutJob::default()
        }
    }

    /// 将LayoutJob返回到内存池
    fn return_layout_job_to_pool(&mut self, job: egui::text::LayoutJob) {
        // 限制内存池大小，避免无限增长
        if self.layout_job_pool.len() < 100 {
            self.layout_job_pool.push(job);
        }
    }

    /// 并行更新缓存的行（使用rayon进行并行处理）
    fn update_cached_lines_parallel(&mut self, lines_to_update: Vec<(usize, String)>) {
        let font_id = egui::FontId::monospace(12.0);
        
        // 创建独立的语法高亮器实例用于并行处理
        let highlighter = SyntaxHighlighter::new();
        
        // 转换为引用格式用于并行解析
        let lines_refs: Vec<(usize, &str)> = lines_to_update.iter()
            .map(|(idx, s)| (*idx, s.as_str()))
            .collect();
        
        // 使用并行解析
        let parsed_lines = highlighter.parse_lines_parallel(&lines_refs);
        
        for (line_idx, tokens) in parsed_lines {
            let mut job = self.get_layout_job_from_pool();

            for token in tokens {
                job.append(
                    &token.text,
                    0.0,
                    egui::TextFormat {
                        font_id: font_id.clone(),
                        color: token.color,
                        ..Default::default()
                    },
                );
            }

            if line_idx < self.cached_highlighted_lines.len() {
                // 将旧的LayoutJob返回到内存池
                let old_job = std::mem::replace(&mut self.cached_highlighted_lines[line_idx], job);
                self.return_layout_job_to_pool(old_job);
            } else {
                self.cached_highlighted_lines.push(job);
            }
        }
    }

    /// 计算代码哈希值
    fn calculate_code_hash(code: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        code.hash(&mut hasher);
        hasher.finish()
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