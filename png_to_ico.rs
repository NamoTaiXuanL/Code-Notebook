// 编码：UTF-8
// PNG转ICO工具

use image::{ImageFormat, RgbaImage};
use std::fs::File;
use std::io::Write;

fn main() {
    // 读取PNG图标
    let img = image::open("Assets/Resources/LOGO/LOGO.png").expect("无法读取PNG图标");

    // 创建多个尺寸的图标
    let sizes = vec![16, 32, 48, 64, 128, 256];
    let mut ico_data: Vec<u8> = Vec::new();

    // ICO文件头
    ico_data.extend_from_slice(&[0u8, 0]); // 保留字段
    ico_data.extend_from_slice(&[1u8, 0]); // 图标类型 (1=ICO)
    ico_data.extend_from_slice(&[(sizes.len() as u8), 0]); // 图像数量

    // 记录图像数据偏移位置
    let mut data_offset = 6 + (sizes.len() * 16) as u32;
    let mut image_entries = Vec::new();

    for &size in &sizes {
        // 调整图像大小
        let resized = img.resize(size as u32, size as u32, image::imageops::FilterType::Lanczos3);
        let rgba = resized.to_rgba8();

        // 转换为BMP格式（ICO使用BMP格式存储）
        let mut bmp_data = create_bmp_data(&rgba);

        // 创建目录条目
        let mut entry = Vec::new();
        entry.push(size as u8);
        entry.push(size as u8);
        entry.push(0); // 调色板
        entry.push(0); // 保留
        entry.push(1); // 颜色平面
        entry.push(32); // 位深度
        entry.extend_from_slice(&(bmp_data.len() as u32).to_le_bytes());
        entry.extend_from_slice(&data_offset.to_le_bytes());

        image_entries.push(entry);
        data_offset += bmp_data.len() as u32;
    }

    // 写入目录条目
    for entry in image_entries {
        ico_data.extend_from_slice(&entry);
    }

    // 写入图像数据
    for &size in &sizes {
        let resized = img.resize(size as u32, size as u32, image::imageops::FilterType::Lanczos3);
        let rgba = resized.to_rgba8();
        let bmp_data = create_bmp_data(&rgba);
        ico_data.extend_from_slice(&bmp_data);
    }

    // 保存ICO文件
    let mut file = File::create("Assets/Resources/LOGO/LOGO.ico").expect("无法创建ICO文件");
    file.write_all(&ico_data).expect("无法写入ICO文件");

    println!("ICO图标创建完成");
}

fn create_bmp_data(rgba: &RgbaImage) -> Vec<u8> {
    let (width, height) = rgba.dimensions();
    let mut bmp_data = Vec::new();

    // BMP信息头 (40字节)
    bmp_data.extend_from_slice(&40u32.to_le_bytes()); // 信息头大小
    bmp_data.extend_from_slice(&width.to_le_bytes());
    bmp_data.extend_from_slice(&(height * 2).to_le_bytes()); // ICO使用双高度
    bmp_data.extend_from_slice(&[1u16, 0].as_ref()); // 颜色平面
    bmp_data.extend_from_slice(&[32u16, 0].as_ref()); // 位深度
    bmp_data.extend_from_slice(&0u32.to_le_bytes()); // 压缩方式
    bmp_data.extend_from_slice(&0u32.to_le_bytes()); // 图像大小
    bmp_data.extend_from_slice(&0i32.to_le_bytes()); // 水平分辨率
    bmp_data.extend_from_slice(&0i32.to_le_bytes()); // 垂直分辨率
    bmp_data.extend_from_slice(&0u32.to_le_bytes()); // 颜色数
    bmp_data.extend_from_slice(&0u32.to_le_bytes()); // 重要颜色

    // BMP数据（从下到上，BGR格式）
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = rgba.get_pixel(x, y);
            bmp_data.push(pixel[2]); // B
            bmp_data.push(pixel[1]); // G
            bmp_data.push(pixel[0]); // R
            bmp_data.push(pixel[3]); // A
        }
    }

    // 添加掩码数据（1位，对于32位图标通常是空的）
    let row_size = ((width + 31) / 32) * 4;
    let mask_size = row_size * height;
    bmp_data.extend_from_slice(&vec![0u8; mask_size as usize]);

    bmp_data
}