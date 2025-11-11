// 编码：UTF-8
// 创建简单的32x32像素图标

use image::{ImageBuffer, Rgba};
use std::fs::File;
use std::io::Write;

fn main() {
    // 创建一个32x32的简单图标
    let mut img = ImageBuffer::new(32, 32);

    // 创建一个简单的渐变背景色
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x * 255 / 31) as u8;
        let g = (y * 255 / 31) as u8;
        let b = 128;
        *pixel = Rgba([r, g, b, 255]);
    }

    // 在中心添加一个简单的方形
    for x in 8..24 {
        for y in 8..24 {
            img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }

    for x in 12..20 {
        for y in 12..20 {
            img.put_pixel(x, y, Rgba([0, 100, 200, 255]));
        }
    }

    // 保存为PNG
    img.save("Assets/Resources/LOGO/LOGO_small.png").unwrap();
    println!("小图标创建完成");
}