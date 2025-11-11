use eframe::egui;
use phf::phf_set;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// 编译时生成的完美哈希关键字集合
static RUST_KEYWORDS: phf::Set<&'static str> = phf_set! {
    "fn", "let", "mut", "pub", "priv", "struct", "impl", "trait", "enum",
    "if", "else", "match", "for", "while", "loop", "break", "continue",
    "use", "mod", "crate", "super", "self", "Self", "return", "async",
    "await", "move", "const", "static", "type", "where", "in",
};

pub struct SyntaxHighlighter {
    cache: HashMap<usize, (u64, Vec<CachedToken>)>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    // 计算行的哈希值用于缓存检测
    fn compute_line_hash(&self, line: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        line.hash(&mut hasher);
        hasher.finish()
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
    pub fn layout_job_line(&mut self, line_number: usize, line: &str) -> egui::text::LayoutJob {
        let mut job = egui::text::LayoutJob::default();

        let tokens = self.parse_line_with_cache(line_number, line);

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

    // 带缓存的解析方法
    pub fn parse_line_with_cache(&mut self, line_number: usize, line: &str) -> Vec<CachedToken> {
        let line_hash = self.compute_line_hash(line);
        
        // 检查缓存中是否有该行的有效结果
        if let Some((cached_hash, cached_tokens)) = self.cache.get(&line_number) {
            if *cached_hash == line_hash {
                return cached_tokens.clone();
            }
        }
        
        // 缓存未命中或哈希不匹配，重新解析
        let tokens = self.parse_line_public(line);
        
        // 转换为缓存Token格式
        let cached_tokens: Vec<CachedToken> = tokens.iter().map(|token| CachedToken {
            text: token.text.to_string(),
            start_col: token.start_col,
            end_col: token.end_col,
            color: token.color,
        }).collect();
        
        // 更新缓存
        self.cache.insert(line_number, (line_hash, cached_tokens.clone()));
        
        cached_tokens
    }

    // 清除特定行的缓存
    pub fn invalidate_line_cache(&mut self, line_number: usize) {
        self.cache.remove(&line_number);
    }

    // 清除所有缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn parse_line_public<'a>(&self, line: &'a str) -> Vec<Token<'a>> {
        // 预分配token向量，假设平均每行有10个token
        let mut tokens = Vec::with_capacity(10);
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
                    let color = if RUST_KEYWORDS.contains(word) {
                        egui::Color32::from_rgb(255, 140, 0) // 橙色关键字
                    } else if word == "true" || word == "false" {
                        egui::Color32::from_rgb(0, 128, 0) // 绿色布尔值
                    } else {
                        egui::Color32::from_rgb(0, 100, 200) // 蓝色标识符
                    };

                    tokens.push(Token {
                        text: word,
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
                        text: whitespace,
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
                        text: string_lit,
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
                        text: char_lit,
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
                        text: number,
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
                            while let Some(&(next_idx, next_ch)) = chars.peek() {
                                end = next_idx + next_ch.len_utf8(); // 正确处理UTF-8字符
                                chars.next();
                            }

                            let comment = &line[start..end];
                            tokens.push(Token {
                                text: comment,
                                start_col: start,
                                end_col: end,
                                color: egui::Color32::from_rgb(100, 100, 100), // 灰色注释
                            });
                            continue;
                        }
                    }

                    // 普通除号或运算符
                    // 单个字符需要转换为字符串切片
                    let char_str = &line[start_idx..start_idx + ch.len_utf8()];
                    tokens.push(Token {
                        text: char_str,
                        start_col: start_idx,
                        end_col: start_idx + ch.len_utf8(),
                        color: egui::Color32::from_rgb(200, 100, 0), // 棕色运算符
                    });
                }
                ch if "+-*/%=&|<>!^".contains(ch) => {
                    // 运算符
                    // 单个字符需要转换为字符串切片
                    let char_str = &line[start_idx..start_idx + ch.len_utf8()];
                    tokens.push(Token {
                        text: char_str,
                        start_col: start_idx,
                        end_col: start_idx + ch.len_utf8(),
                        color: egui::Color32::from_rgb(200, 100, 0), // 棕色运算符
                    });
                }
                ch if "(){}[];:,".contains(ch) => {
                    // 标点符号
                    // 单个字符需要转换为字符串切片
                    let char_str = &line[start_idx..start_idx + ch.len_utf8()];
                    tokens.push(Token {
                        text: char_str,
                        start_col: start_idx,
                        end_col: start_idx + ch.len_utf8(),
                        color: egui::Color32::from_rgb(50, 50, 50), // 深灰色标点
                    });
                }
                _ => {
                    // 其他字符
                    // 单个字符需要转换为字符串切片
                    let char_str = &line[start_idx..start_idx + ch.len_utf8()];
                    tokens.push(Token {
                        text: char_str,
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

// 用于缓存的Token结构，拥有字符串数据
#[derive(Clone)]
pub struct CachedToken {
    pub text: String,  // 拥有字符串数据
    #[allow(dead_code)]
    pub start_col: usize,
    #[allow(dead_code)]
    pub end_col: usize,
    pub color: egui::Color32,
}