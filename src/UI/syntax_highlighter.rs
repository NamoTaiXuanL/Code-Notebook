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

    // 为了兼容性保留旧方法，但不使用
    #[allow(dead_code)]
    pub fn layout_job(&self, code: &str) -> egui::text::LayoutJob {
        let mut job = egui::text::LayoutJob::default();

        for line in code.lines() {
            let tokens = self.parse_line_public(line);

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

    // 为了兼容性保留旧方法，但不使用
    #[allow(dead_code)]
    pub fn layout_job_line(&self, line: &str) -> egui::text::LayoutJob {
        let mut job = egui::text::LayoutJob::default();

        let tokens = self.parse_line_public(line);

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
    pub fn paint_highlights(&self, _painter: &egui::Painter, _rect: egui::Rect, _code: &str, _char_width: f32, _line_height: f32) {
        // 这个方法现在不用了，我们改用 LayoutJob
    }

    pub fn parse_line_public(&self, line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = line.char_indices().peekable();

        while let Some((start_idx, ch)) = chars.next() {
            match ch {
                ch if ch.is_alphabetic() || ch == '_' => {
                    // 标识符或关键字 - 使用切片避免字符串复制
                    let start = start_idx;
                    let mut end = start_idx + ch.len_utf8();

                    // 收集连续的字母、数字或下划线
                    while let Some(&(next_idx, next_ch)) = chars.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '_' {
                            end = next_idx + next_ch.len_utf8();
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    let word = &line[start..end];
                    let color = if self.rust_keywords.contains(word) {
                        egui::Color32::from_rgb(255, 140, 0) // 橙色关键字
                    } else if word == "true" || word == "false" {
                        egui::Color32::from_rgb(0, 128, 0) // 绿色布尔值
                    } else {
                        egui::Color32::from_rgb(0, 100, 200) // 蓝色标识符
                    };

                    tokens.push(Token {
                        text: word.to_string(),
                        start_col: start,
                        end_col: end,
                        color,
                    });
                }
                ch if ch.is_whitespace() => {
                    // 空白字符 - 使用切片避免字符串复制
                    let start = start_idx;
                    let mut end = start_idx + ch.len_utf8();

                    // 收集连续的空白字符
                    while let Some(&(next_idx, next_ch)) = chars.peek() {
                        if next_ch.is_whitespace() {
                            end = next_idx + next_ch.len_utf8();
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    let whitespace = &line[start..end];
                    tokens.push(Token {
                        text: whitespace.to_string(),
                        start_col: start,
                        end_col: end,
                        color: egui::Color32::GRAY, // 灰色空白字符
                    });
                }
                '"' => {
                    // 字符串字面量 - 使用切片避免字符串复制
                    let start = start_idx;
                    let mut end = start_idx + ch.len_utf8();
                    let mut is_escaped = false;

                    while let Some(&(next_idx, next_ch)) = chars.peek() {
                        end = next_idx + next_ch.len_utf8();
                        chars.next();

                        if next_ch == '\\' && !is_escaped {
                            is_escaped = true;
                            continue;
                        }

                        if next_ch == '"' && !is_escaped {
                            break;
                        }

                        is_escaped = false;
                    }

                    let string_lit = &line[start..end];
                    tokens.push(Token {
                        text: string_lit.to_string(),
                        start_col: start,
                        end_col: end,
                        color: egui::Color32::from_rgb(0, 128, 0), // 绿色字符串
                    });
                }
                '\'' => {
                    // 字符字面量 - 使用切片避免字符串复制
                    let start = start_idx;
                    let mut end = start_idx + ch.len_utf8();
                    let mut is_escaped = false;

                    while let Some(&(next_idx, next_ch)) = chars.peek() {
                        end = next_idx + next_ch.len_utf8();
                        chars.next();

                        if next_ch == '\\' && !is_escaped {
                            is_escaped = true;
                            continue;
                        }

                        if next_ch == '\'' && !is_escaped {
                            break;
                        }

                        is_escaped = false;
                    }

                    let char_lit = &line[start..end];
                    tokens.push(Token {
                        text: char_lit.to_string(),
                        start_col: start,
                        end_col: end,
                        color: egui::Color32::from_rgb(0, 128, 0), // 绿色字符
                    });
                }
                ch if ch.is_ascii_digit() => {
                    // 数字字面量 - 使用切片避免字符串复制
                    let start = start_idx;
                    let mut end = start_idx + ch.len_utf8();

                    while let Some(&(next_idx, next_ch)) = chars.peek() {
                        if next_ch.is_ascii_digit() || next_ch == '.' || next_ch == '_' {
                            end = next_idx + next_ch.len_utf8();
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    let number = &line[start..end];
                    tokens.push(Token {
                        text: number.to_string(),
                        start_col: start,
                        end_col: end,
                        color: egui::Color32::from_rgb(128, 0, 128), // 紫色数字
                    });
                }
                '/' => {
                    // 可能是注释
                    if let Some(&(_, next_ch)) = chars.peek() {
                        if next_ch == '/' {
                            // 单行注释 - 使用切片避免字符串复制
                            let start = start_idx;
                            let mut end = start_idx + ch.len_utf8();
                            end += chars.next().unwrap().1.len_utf8(); // 消耗第二个 '/'

                            // 消耗该行剩余所有字符
                            while let Some(&(next_idx, _)) = chars.peek() {
                                end = next_idx + 1; // 假设ASCII字符
                                chars.next();
                            }

                            let comment = &line[start..end];
                            tokens.push(Token {
                                text: comment.to_string(),
                                start_col: start,
                                end_col: end,
                                color: egui::Color32::from_rgb(100, 100, 100), // 灰色注释
                            });
                            continue;
                        }
                    }

                    // 普通除号或运算符
                    tokens.push(Token {
                        text: ch.to_string(),
                        start_col: start_idx,
                        end_col: start_idx + ch.len_utf8(),
                        color: egui::Color32::from_rgb(200, 100, 0), // 棕色运算符
                    });
                }
                ch if "+-*/%=&|<>!^".contains(ch) => {
                    // 运算符
                    tokens.push(Token {
                        text: ch.to_string(),
                        start_col: start_idx,
                        end_col: start_idx + ch.len_utf8(),
                        color: egui::Color32::from_rgb(200, 100, 0), // 棕色运算符
                    });
                }
                ch if "(){}[];:,".contains(ch) => {
                    // 标点符号
                    tokens.push(Token {
                        text: ch.to_string(),
                        start_col: start_idx,
                        end_col: start_idx + ch.len_utf8(),
                        color: egui::Color32::from_rgb(50, 50, 50), // 深灰色标点
                    });
                }
                _ => {
                    // 其他字符
                    tokens.push(Token {
                        text: ch.to_string(),
                        start_col: start_idx,
                        end_col: start_idx + ch.len_utf8(),
                        color: egui::Color32::DARK_GRAY,
                    });
                }
            }
        }

        tokens
    }
}

pub struct Token<'a> {
    pub text: &'a str,  // 使用字符串切片引用，避免复制
    #[allow(dead_code)]
    pub start_col: usize,
    #[allow(dead_code)]
    pub end_col: usize,
    pub color: egui::Color32,
}