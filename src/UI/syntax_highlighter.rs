use eframe::egui;
use std::collections::HashSet;

pub struct SyntaxHighlighter {
    rust_keywords: HashSet<&'static str>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let mut rust_keywords = HashSet::new();
        rust_keywords.extend([
            "fn", "let", "mut", "pub", "priv", "struct", "impl", "trait", "enum",
            "if", "else", "match", "for", "while", "loop", "break", "continue",
            "use", "mod", "crate", "super", "self", "Self", "return", "async",
            "await", "move", "const", "static", "type", "where", "in",
        ]);

        Self { rust_keywords }
    }

    // 使用 LayoutJob API 创建语法高亮的文本布局
    pub fn layout_job(&self, code: &str) -> egui::text::LayoutJob {
        let mut job = egui::text::LayoutJob::default();

        for line in code.lines() {
            let tokens = self.parse_line(line);

            for token in tokens {
                job.append(
                    &token.text,
                    0.0,  // text_offset
                    egui::TextFormat {
                        font_id: egui::FontId::monospace(12.0),
                        color: token.color,
                        valign: egui::Align::Center,
                        ..Default::default()
                    },
                );
            }

            // 添加换行符（如果不是最后一行）
            job.append(
                "\n",
                0.0,
                egui::TextFormat {
                    font_id: egui::FontId::monospace(12.0),
                    color: egui::Color32::GRAY,
                    ..Default::default()
                },
            );
        }

        job
    }

    // 渲染单行的语法高亮 - 用于优化大文件性能
    pub fn layout_job_line(&self, line: &str) -> egui::text::LayoutJob {
        let mut job = egui::text::LayoutJob::default();

        let tokens = self.parse_line(line);

        for token in tokens {
            job.append(
                &token.text,
                0.0,  // text_offset
                egui::TextFormat {
                    font_id: egui::FontId::monospace(12.0),
                    color: token.color,
                    valign: egui::Align::Center,
                    ..Default::default()
                },
            );
        }

        job
    }

    // 为了兼容性保留旧方法，但不使用
    #[allow(dead_code)]
    pub fn paint_highlights(&self, painter: &egui::Painter, rect: egui::Rect, code: &str, char_width: f32, line_height: f32) {
        // 这个方法现在不用了，我们改用 LayoutJob
    }

    pub fn parse_line_public(&self, line: &str) -> Vec<Token> {
        self.parse_line(line)
    }

    fn parse_line(&self, line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = line.char_indices().peekable();

        while let Some((start_idx, ch)) = chars.next() {
            let start_col = line[..start_idx].chars().count();

            match ch {
                ch if ch.is_alphabetic() || ch == '_' => {
                    // 标识符或关键字
                    let mut word = String::new();
                    word.push(ch);

                    while let Some(&(_, next_ch)) = chars.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '_' {
                            word.push(chars.next().unwrap().1);
                        } else {
                            break;
                        }
                    }

                    let color = if self.rust_keywords.contains(word.as_str()) {
                        egui::Color32::from_rgb(255, 140, 0) // 橙色关键字
                    } else if word == "true" || word == "false" {
                        egui::Color32::from_rgb(0, 128, 0) // 绿色布尔值
                    } else {
                        egui::Color32::from_rgb(0, 100, 200) // 蓝色标识符
                    };

                    let end_col = start_col + word.chars().count();
                    tokens.push(Token {
                        text: word,
                        start_col,
                        end_col,
                        color,
                    });
                }
                ch if ch.is_whitespace() => {
                    // 保留空白字符，避免缩进丢失
                    let mut whitespace = String::new();
                    whitespace.push(ch);

                    // 收集连续的空白字符
                    while let Some(&(_, next_ch)) = chars.peek() {
                        if next_ch.is_whitespace() {
                            whitespace.push(chars.next().unwrap().1);
                        } else {
                            break;
                        }
                    }

                    let end_col = start_col + whitespace.chars().count();
                    tokens.push(Token {
                        text: whitespace,
                        start_col,
                        end_col,
                        color: egui::Color32::GRAY, // 灰色空白字符
                    });
                }
                '"' => {
                    // 字符串字面量
                    let mut string_lit = String::new();
                    string_lit.push(ch);
                    let mut is_escaped = false;

                    while let Some(&(_, next_ch)) = chars.peek() {
                        let ch = chars.next().unwrap().1;
                        string_lit.push(ch);

                        if ch == '\\' && !is_escaped {
                            is_escaped = true;
                            continue;
                        }

                        if ch == '"' && !is_escaped {
                            break;
                        }

                        is_escaped = false;
                    }

                    let end_col = start_col + string_lit.chars().count();
                    tokens.push(Token {
                        text: string_lit,
                        start_col,
                        end_col,
                        color: egui::Color32::from_rgb(0, 128, 0), // 绿色字符串
                    });
                }
                '\'' => {
                    // 字符字面量
                    let mut char_lit = String::new();
                    char_lit.push(ch);
                    let mut is_escaped = false;

                    while let Some(&(_, next_ch)) = chars.peek() {
                        let ch = chars.next().unwrap().1;
                        char_lit.push(ch);

                        if ch == '\\' && !is_escaped {
                            is_escaped = true;
                            continue;
                        }

                        if ch == '\'' && !is_escaped {
                            break;
                        }

                        is_escaped = false;
                    }

                    let end_col = start_col + char_lit.chars().count();
                    tokens.push(Token {
                        text: char_lit,
                        start_col,
                        end_col,
                        color: egui::Color32::from_rgb(0, 128, 0), // 绿色字符
                    });
                }
                ch if ch.is_ascii_digit() => {
                    // 数字字面量
                    let mut number = String::new();
                    number.push(ch);

                    while let Some(&(_, next_ch)) = chars.peek() {
                        if next_ch.is_ascii_digit() || next_ch == '.' || next_ch == '_' {
                            number.push(chars.next().unwrap().1);
                        } else {
                            break;
                        }
                    }

                    let end_col = start_col + number.chars().count();
                    tokens.push(Token {
                        text: number,
                        start_col,
                        end_col,
                        color: egui::Color32::from_rgb(128, 0, 128), // 紫色数字
                    });
                }
                '/' => {
                    // 可能是注释
                    if let Some(&(_, next_ch)) = chars.peek() {
                        if next_ch == '/' {
                            // 单行注释
                            let mut comment = String::new();
                            comment.push(ch);
                            comment.push(chars.next().unwrap().1);

                            // 消耗该行剩余所有字符
                            while let Some(&(_, next_ch)) = chars.peek() {
                                comment.push(chars.next().unwrap().1);
                            }

                            let end_col = start_col + comment.chars().count();
                            tokens.push(Token {
                                text: comment,
                                start_col,
                                end_col,
                                color: egui::Color32::from_rgb(100, 100, 100), // 灰色注释
                            });
                            continue;
                        }
                    }

                    // 普通除号或运算符
                    tokens.push(Token {
                        text: ch.to_string(),
                        start_col,
                        end_col: start_col + 1,
                        color: egui::Color32::from_rgb(200, 100, 0), // 棕色运算符
                    });
                }
                ch if "+-*/%=&|<>!^".contains(ch) => {
                    // 运算符
                    tokens.push(Token {
                        text: ch.to_string(),
                        start_col,
                        end_col: start_col + 1,
                        color: egui::Color32::from_rgb(200, 100, 0), // 棕色运算符
                    });
                }
                ch if "(){}[];:,".contains(ch) => {
                    // 标点符号
                    tokens.push(Token {
                        text: ch.to_string(),
                        start_col,
                        end_col: start_col + 1,
                        color: egui::Color32::from_rgb(50, 50, 50), // 深灰色标点
                    });
                }
                _ => {
                    // 其他字符
                    tokens.push(Token {
                        text: ch.to_string(),
                        start_col,
                        end_col: start_col + 1,
                        color: egui::Color32::DARK_GRAY,
                    });
                }
            }
        }

        tokens
    }
}

pub struct Token {
    pub text: String,
    pub start_col: usize,
    pub end_col: usize,
    pub color: egui::Color32,
}