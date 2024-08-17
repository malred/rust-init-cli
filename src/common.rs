use std::{fs, io};
use std::path::Path;

// 复制文件夹到指定路径
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    // fs::create_dir_all(&dst)?;
    fs::create_dir_all(&dst).expect("创建dst目录失败");

    // println!("遍历");
    fs::read_dir(&src).expect("找不到src目录");
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

// 读取输入
pub fn read_line() -> String {
    let mut input = String::new();
    // 遇到错误, 不关心, 只希望它遇到错误就停止(故障时执行panic)
    // std::io::stdin().read_line(&mut input).unwrap();
    // 与 unwrap 相比，expect 提供了相同的功能，但允许附加一个自定义的错误消息
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}