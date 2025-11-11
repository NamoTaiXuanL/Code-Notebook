// 编码：UTF-8
// 作者：code_notebook项目组Seraphiel

use eframe::egui;
use std::path::PathBuf;

mod ui;
use ui::layout::MainLayout;
use ui::styles;

#[derive(Clone)]
pub struct DirectoryItem {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
}

/// 应用状态
pub struct AppState {
    pub code: String,           // 代码内容
    pub file_path: Option<PathBuf>, // 文件路径
    pub status: String,        // 状态信息
    pub current_directory: PathBuf, // 当前显示的目录
    pub directory_items: Vec<DirectoryItem>, // 目录内容列表
}

impl Default for AppState {
    fn default() -> Self {
        let mut state = Self {
            code: "将代码文件拖拽到窗口即可查看".to_string(),
            file_path: None,
            status: String::new(),
            current_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            directory_items: Vec::new(),
        };

        // 加载初始目录内容
        state.load_directory_content();
        state
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 处理拖拽文件
        self.handle_dropped_files(ctx);

        // 创建布局并渲染（优化：减少状态复制）
        let mut main_layout = MainLayout::new(self);

        // 渲染UI并获取可能的文件加载请求
        if let Some(file_path) = main_layout.render(ctx, frame, self) {
            // 文件加载后直接更新状态，避免下一帧的额外开销
            self.load_file(file_path);
        }
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

                // 设置当前目录为文件所在目录
                if let Some(parent_dir) = path.parent() {
                    self.current_directory = parent_dir.to_path_buf();
                    self.load_directory_content();
                }
            }
            Err(e) => {
                self.code = format!("读取失败: {}", e);
                self.status = "错误".to_string();
                self.file_path = None;
                self.directory_items.clear();
            }
        }
    }

    /// 加载当前目录的内容
    fn load_directory_content(&mut self) {
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
            styles::setup_chinese_fonts(&cc.egui_ctx);
            Box::new(initial_state)
        }),
    )
}