// 编码：UTF-8
// 构建脚本：配置Windows子系统和资源嵌入

fn main() {
    #[cfg(target_os = "windows")]
    {
        // 设置Windows子系统为GUI，隐藏控制台窗口
        println!("cargo:rustc-link-arg-bin=code_notebook=/SUBSYSTEM:WINDOWS");
        println!("cargo:rustc-link-arg-bin=code_notebook=/ENTRY:mainCRTStartup");

        // 编译资源文件
        embed_resource::compile("app.rc", embed_resource::NONE);
    }
}